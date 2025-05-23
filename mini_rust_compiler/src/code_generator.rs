use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

use crate::error_handler::ErrorHandler;
use crate::parser::{Program, Function, Stmt, Expr, BinaryOp, Literal, Type};

// Structure pour stocker les informations sur les variables
#[derive(Clone)]
struct VarInfo {
    offset: usize,
    var_type: Type,
}

pub struct CodeGenerator<'a> {
    error_handler: &'a ErrorHandler,
    current_function: Option<String>,
    label_counter: usize,  // Utile pour générer des étiquettes uniques
    variable_info: HashMap<String, VarInfo>,
    function_params: HashMap<String, Vec<String>>,
    format_labels: HashMap<(String, usize), String>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(error_handler: &'a ErrorHandler) -> Self {
        CodeGenerator { 
            error_handler,
            current_function: None,
            label_counter: 0,
            variable_info: HashMap::new(),
            function_params: HashMap::new(),
            format_labels: HashMap::new(),
        }
    }
    
    pub fn generate(&mut self, program: Program, source_path: &str) -> Result<String, usize> {
        // Générer le code assembleur pour le programme
        let asm_code = self.generate_asm_code(&program)?;
        
        // Déterminer le nom de l'exécutable
        let source_path = Path::new(source_path);
        let stem = if let Some(stem) = source_path.file_stem() {
            stem.to_str().unwrap().to_string()
        } else {
            "a.out".to_string()
        };
        
        // Créer un fichier assembleur intermédiaire
        let asm_file_path = format!("{}.asm", stem);
        let obj_file_path = format!("{}.o", stem);
        
        let mut asm_file = match File::create(&asm_file_path) {
            Ok(file) => file,
            Err(e) => {
                self.error_handler.report_error(0, &format!("Erreur lors de la création du fichier assembleur: {}", e));
                return Err(0);
            }
        };
        
        if let Err(e) = asm_file.write_all(asm_code.as_bytes()) {
            self.error_handler.report_error(0, &format!("Erreur lors de l'écriture dans le fichier assembleur: {}", e));
            return Err(0);
        }
        
        println!("Code assembleur généré avec succès dans le fichier {}", asm_file_path);
        
        // Assembler le code en fichier objet
        let nasm_output = match Command::new("nasm")
            .arg("-f")
            .arg("elf64")
            .arg("-o")
            .arg(&obj_file_path)
            .arg(&asm_file_path)
            .output() {
                Ok(output) => output,
                Err(e) => {
                    self.error_handler.report_error(0, &format!("Erreur lors de l'exécution de nasm: {}", e));
                    return Err(0);
                }
            };
        
        if !nasm_output.status.success() {
            let error = String::from_utf8_lossy(&nasm_output.stderr);
            self.error_handler.report_error(0, &format!("Erreur d'assemblage: {}", error));
            return Err(0);
        }
        
        println!("Assemblage réussi: {}", obj_file_path);
        
        // Lier le fichier objet en exécutable en incluant la libc
        let ld_output = match Command::new("gcc")  // Utiliser gcc au lieu de ld directement
            .arg("-o")
            .arg(&stem)
            .arg(&obj_file_path)
            .arg("-no-pie")  // Désactiver PIE pour simplifier le code assembleur
            .output() {
                Ok(output) => output,
                Err(e) => {
                    self.error_handler.report_error(0, &format!("Erreur lors de l'exécution de gcc pour l'édition de liens: {}", e));
                    return Err(0);
                }
            };
        
        if !ld_output.status.success() {
            let error = String::from_utf8_lossy(&ld_output.stderr);
            self.error_handler.report_error(0, &format!("Erreur d'édition de liens: {}", error));
            return Err(0);
        }
        
        println!("Édition de liens réussie! Exécutable généré: {}", stem);
        
        // Optionnel : Suppression des fichiers intermédiaires
        // std::fs::remove_file(&asm_file_path).ok();
        // std::fs::remove_file(&obj_file_path).ok();
        
        Ok(stem)
    }
    
    fn type_size(&self, typ: &Type) -> usize {
        match typ {
            Type::I8 => 1,
            Type::I16 => 2,
            Type::I32 => 4,
            Type::I64 => 8,
            Type::I128 => 16,
            Type::F32 => 4,
            Type::F64 => 8,
            Type::String => 8, // Stocké comme un pointeur
            Type::Void => 0,
        }
    }
    
    fn align_to_8_bytes(size: usize) -> usize {
        (size + 7) & !7  // Arrondir au multiple de 8 supérieur
    }
    
    fn generate_asm_code(&mut self, program: &Program) -> Result<String, usize> {
        let mut code = String::new();
        
        // Préparation - Collecter tous les paramètres de fonction
        for function in &program.functions {
            let param_names: Vec<String> = function.params.iter()
                .map(|(name, _)| name.clone())
                .collect();
            self.function_params.insert(function.name.clone(), param_names);
        }
        
        // En-tête assembleur
        code.push_str("section .data\n");
        
        // Constantes et variables globales
        code.push_str("    format_integer db \"%d\", 0\n");
        code.push_str("    format_string db \"%s\", 0\n");
        code.push_str("    newline db 10, 0\n");
        
        // Constantes pour println!
        // Premier passage pour générer des étiquettes stables et les stocker dans un dictionnaire
        self.format_labels.clear();
        for function in &program.functions {
            self.process_function_for_format_labels(function);
        }
        
        // Maintenant générons les définitions de chaînes avec les étiquettes stables
        for function in &program.functions {
            self.generate_format_strings_for_function(function, &mut code);
        }
        
        // Section de code
        code.push_str("\nsection .text\n");
        // Ne pas définir _start quand on utilise gcc comme éditeur de liens
        code.push_str("    extern printf\n");
        code.push_str("    extern exit\n");
        code.push_str("    global main\n\n");  // Définir main comme global pour l'édition de liens
        
        // Implémentation des fonctions (sauf main qui sera traité spécialement)
        for function in &program.functions {
            if function.name != "main" {  // Générer toutes les fonctions sauf main
                self.variable_info.clear();
                code.push_str(&self.generate_function(function)?);
            }
        }
        
        // Traitement spécial pour main pour qu'il suive la convention d'appel C
        if let Some(main_function) = program.functions.iter().find(|f| f.name == "main") {
            self.variable_info.clear();
            self.current_function = Some("main".to_string());
            
            code.push_str("main:\n");
            code.push_str("    push rbp\n");
            code.push_str("    mov rbp, rsp\n");
            
            // Count all variables including loop variables
            let mut total_vars = 0;
            self.count_all_variables(&main_function.body, &mut total_vars);
            
            // Add some extra space for safety and alignment
            let local_vars_size = (total_vars + 2) * 8; // Add 2 extra slots
            
            println!("Found {} total variables (including loop vars), allocating {} bytes", total_vars, local_vars_size);
            
            if local_vars_size > 0 {
                // Ensure stack is aligned to 16 bytes (required by System V ABI)
                let aligned_size = ((local_vars_size + 15) / 16) * 16;
                code.push_str(&format!("    sub rsp, {}\n", aligned_size));
            }
            
            // Enregistrer les décalages des variables locales
            let mut offset = 0;
            self.assign_variable_offsets(&main_function.body, &mut offset);
            
            // Debug: Print variable assignments
            println!("Variable assignments:");
            for (name, info) in &self.variable_info {
                println!("  {} -> offset {}", name, info.offset);
            }
            
            // Générer le code du corps de la fonction main
            for (i, stmt) in main_function.body.iter().enumerate() {
                let stmt_code = self.generate_statement(stmt, i)?;
                code.push_str(&stmt_code);
            }
            
            // Épilogue - retourner 0
            code.push_str("\n    ; Épilogue de main avec valeur de retour 0\n");
            code.push_str("    mov eax, 0\n");  // Valeur de retour 0 pour indiquer la réussite
            code.push_str("    mov rsp, rbp\n");
            code.push_str("    pop rbp\n");
            code.push_str("    ret\n\n");
            
            self.current_function = None;
        }
        
        Ok(code)
    }
    
    fn generate_function(&mut self, function: &Function) -> Result<String, usize> {
        self.current_function = Some(function.name.clone());
        self.variable_info.clear();
        
        let mut code = String::new();
        
        // Étiquette de la fonction
        code.push_str(&format!("{}:\n", function.name));
        
        // Prologue de la fonction
        code.push_str("    push rbp\n");
        code.push_str("    mov rbp, rsp\n");
        
        // Enregistrer les paramètres de la fonction
        if let Some(param_names) = self.function_params.get(&function.name) {
            let registers = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
            
            // Calculer l'espace nécessaire pour les variables locales et les paramètres
            let total_local_vars = function.body.iter()
                .filter(|stmt| matches!(stmt, Stmt::Let(_, _, _, _)))
                .count();
            
            let stack_size = (total_local_vars + param_names.len()) * 8;
            if stack_size > 0 {
                code.push_str(&format!("    sub rsp, {}\n", stack_size));
            }
            
            // Sauvegarder les paramètres sur la pile
            for (i, param_name) in param_names.iter().enumerate() {
                let offset = (i + 1) * 8;
                self.variable_info.insert(param_name.clone(), VarInfo {
                    offset,
                    var_type: Type::I32, // Default type for parameters
                });
                
                if i < registers.len() {
                    code.push_str(&format!("    mov QWORD [rbp-{}], {}\n", offset, registers[i]));
                } else {
                    // Les paramètres supplémentaires sont déjà sur la pile
                    // Note: ceci est simplifié, il faudrait gérer la convention d'appel correctement
                    code.push_str(&format!("    ; Paramètre {} sur la pile\n", param_name));
                }
            }
            
            // Maintenant, réserver de l'espace pour les variables locales
            let mut current_offset = param_names.len() * 8;
            
            for stmt in &function.body {
                if let Stmt::Let(name, _, _, var_type) = stmt {
                    let var_size = self.type_size(var_type);
                    current_offset = Self::align_to_8_bytes(current_offset + var_size);
                    
                    self.variable_info.insert(name.clone(), VarInfo { 
                        offset: current_offset, 
                        var_type: var_type.clone() 
                    });
                }
            }
        } else {
            // Pas de paramètres, juste gérer les variables locales
            let local_vars_size = function.body.iter()
                .filter(|stmt| matches!(stmt, Stmt::Let(_, _, _, _)))
                .count() * 8;
            
            if local_vars_size > 0 {
                code.push_str(&format!("    sub rsp, {}\n", local_vars_size));
            }
            
            // Enregistrer les décalages des variables locales
            let mut offset = 0;
            for stmt in &function.body {
                if let Stmt::Let(name, _, _, var_type) = stmt {
                    let var_size = self.type_size(var_type);
                    offset = Self::align_to_8_bytes(offset + var_size);
                    
                    self.variable_info.insert(name.clone(), VarInfo { 
                        offset, 
                        var_type: var_type.clone() 
                    });
                }
            }
        }
        
        // Corps de la fonction
        for (i, stmt) in function.body.iter().enumerate() {
            let stmt_code = self.generate_statement(stmt, i)?;
            code.push_str(&stmt_code);
        }
        
        // Épilogue par défaut si aucun return explicite n'est trouvé
        code.push_str("\n    ; Épilogue de la fonction\n");
        code.push_str("    mov rsp, rbp\n");
        code.push_str("    pop rbp\n");
        code.push_str("    ret\n\n");
        
        self.current_function = None;
        Ok(code)
    }
    
    fn generate_statement(&mut self, stmt: &Stmt, index: usize) -> Result<String, usize> {
        let mut code = String::new();
        
        match stmt {
            Stmt::Let(name, initializer, _mutable, _var_type) => {
                let var_info = self.variable_info.get(name).cloned().unwrap_or_else(|| {
                    self.error_handler.report_error(0, &format!("Variable non trouvée: {}", name));
                    VarInfo { offset: 8 * (index + 1), var_type: Type::I32 }
                });
                
                code.push_str(&format!("\n    ; Variable declaration: {}\n", name));
                
                if let Some(init_expr) = initializer {
                    // Évaluer l'expression et la stocker dans rax
                    code.push_str(&self.generate_expr_code(init_expr)?);
                    // Stocker la valeur à l'emplacement approprié
                    code.push_str(&format!("    mov QWORD [rbp-{}], rax\n", var_info.offset));
                }
            },
            Stmt::Return(expr) => {
                code.push_str("\n    ; Return statement\n");
                
                if let Some(ret_expr) = expr {
                    // Évaluer l'expression de retour et la mettre dans rax
                    code.push_str(&self.generate_expr_code(ret_expr)?);
                }
                
                // Épilogue de la fonction
                code.push_str("    mov rsp, rbp\n");
                code.push_str("    pop rbp\n");
                code.push_str("    ret\n");
            },
            Stmt::Println(args) => {
                if args.is_empty() {
                    code.push_str("\n    ; println! (newline only)\n");
                    code.push_str("    mov rdi, newline\n");
                    code.push_str("    call printf\n");
                } else if let Expr::Literal(Literal::String(format_str)) = &args[0] {
                    let unknown_str = "unknown".to_string();
                    let func_name = self.current_function.as_ref().unwrap_or(&unknown_str);
                    
                    code.push_str(&format!("\n    ; println!(\"{}\", ...)\n", format_str));
                    
                    // Stocker l'étiquette dans une variable avant de faire les appels qui empruntent self de manière mutable
                    let label_value = if let Some(label) = self.format_labels.get(&(func_name.clone(), index)) {
                        label.clone()
                    } else {
                        self.error_handler.report_error(0, &format!("Étiquette de format non trouvée pour println! à l'index {}", index));
                        return Err(0);
                    };
                    
                    // Évaluer tous les arguments en premier et les sauvegarder sur la pile
                    // Attention: on empile en ordre inverse pour faciliter leur récupération
                    for arg in args.iter().skip(1).rev() {
                        code.push_str("\n    ; Évaluation d'un argument\n");
                        code.push_str(&self.generate_expr_code(arg)?);
                        code.push_str("    push rax  ; Sauvegarde de l'argument sur la pile\n");
                    }
                    
                    // Registres dans l'ordre standard de la convention d'appel System V AMD64 ABI
                    let registers = ["rsi", "rdx", "rcx", "r8", "r9"];
                    
                    // Récupérer les arguments dans l'ordre correct
                    code.push_str("\n    ; Configuration des registres pour printf\n");
                    for (i, _) in args.iter().skip(1).enumerate() {
                        if i < registers.len() {
                            code.push_str(&format!("    pop {}  ; Argument {}\n", registers[i], i + 1));
                        } else {
                            // Arguments supplémentaires restent sur la pile pour printf
                            break;
                        }
                    }
                    
                    // Charger l'adresse du format en dernier pour ne pas l'écraser
                    code.push_str(&format!("    lea rdi, [rel {}]  ; Format string\n", label_value));
                    
                    // Appel à printf
                    code.push_str("    xor eax, eax  ; Pas de flottants\n");
                    code.push_str("    call printf\n");
                }
            },
            Stmt::Expression(expr) => {
                code.push_str("\n    ; Expression statement\n");
                code.push_str(&self.generate_expr_code(expr)?);
                // Le résultat est ignoré
            },
            Stmt::For(var_name, range_start, range_end, body) => {
                code.push_str("\n    ; For loop\n");
                
                // Create unique labels for loop control
                let cond_label = format!("L_for_cond_{}", self.label_counter);
                let end_label = format!("L_for_end_{}", self.label_counter);
                self.label_counter += 1;
                
                // Get the variable offset (should already be assigned)
                let var_offset = if let Some(var_info) = self.variable_info.get(var_name) {
                    var_info.offset
                } else {
                    self.error_handler.report_error(0, &format!("Loop variable {} not found in variable_info", var_name));
                    return Err(0);
                };
                
                println!("For loop: variable {} at offset {}", var_name, var_offset);
                
                // Initialize loop variable with range_start
                code.push_str(&self.generate_expr_code(range_start)?);
                code.push_str(&format!("    mov DWORD [rbp-{}], eax  ; Initialize loop variable\n", var_offset));
                
                // Condition check - compare to range_end
                code.push_str(&format!("{}:\n", cond_label));
                code.push_str(&format!("    mov eax, DWORD [rbp-{}]  ; Load counter\n", var_offset));
                code.push_str("    push rax\n"); // Save counter
                code.push_str(&self.generate_expr_code(range_end)?); // Calculate end value
                code.push_str("    mov ecx, eax  ; Move end value to ecx\n");
                code.push_str("    pop rax\n");      // Restore counter to rax
                code.push_str("    cmp eax, ecx  ; Compare counter with end\n");
                code.push_str(&format!("    jge {}  ; Exit if counter >= end\n", end_label));
                
                // Body of the loop
                if let Stmt::Block(stmts) = &**body {
                    for (i, stmt) in stmts.iter().enumerate() {
                        // Use nested indexing for proper statement label management
                        let nested_index = index * 100 + i + 1;
                        code.push_str(&self.generate_statement(stmt, nested_index)?);
                    }
                }
                
                // Increment counter
                code.push_str(&format!("    mov eax, DWORD [rbp-{}]  ; Load counter for increment\n", var_offset));
                code.push_str("    add eax, 1  ; Increment\n");
                code.push_str(&format!("    mov DWORD [rbp-{}], eax  ; Store incremented value\n", var_offset));
                code.push_str(&format!("    jmp {}  ; Jump back to condition\n", cond_label));
                
                // End of loop
                code.push_str(&format!("{}:\n", end_label));
            },
            Stmt::If(condition, then_stmt, else_stmt) => {
                code.push_str("\n    ; If statement\n");
                
                // Create unique labels for if control flow
                let else_label = format!("L_if_else_{}", self.label_counter);
                let end_label = format!("L_if_end_{}", self.label_counter);
                self.label_counter += 1;
                
                // Generate condition evaluation
                code.push_str(&self.generate_expr_code(condition)?);
                
                // Test if condition is false (0)
                code.push_str("    test rax, rax  ; Test if condition is zero\n");
                code.push_str(&format!("    jz {}  ; Jump to else if condition is false\n", 
                    if else_stmt.is_some() { &else_label } else { &end_label }));
                
                // Generate then branch
                code.push_str("\n    ; Then branch\n");
                if let Stmt::Block(stmts) = &**then_stmt {
                    for (i, stmt) in stmts.iter().enumerate() {
                        let nested_index = index * 100 + i + 1;
                        code.push_str(&self.generate_statement(stmt, nested_index)?);
                    }
                } else {
                    let nested_index = index * 100 + 1;
                    code.push_str(&self.generate_statement(then_stmt, nested_index)?);
                }
                
                // Jump to end after then branch (skip else)
                if else_stmt.is_some() {
                    code.push_str(&format!("    jmp {}  ; Skip else branch\n", end_label));
                }
                
                // Generate else branch if it exists
                if let Some(else_stmt) = else_stmt {
                    code.push_str(&format!("{}:\n", else_label));
                    code.push_str("    ; Else branch\n");
                    
                    if let Stmt::Block(stmts) = &**else_stmt {
                        for (i, stmt) in stmts.iter().enumerate() {
                            let nested_index = index * 100 + i + 2; // +2 to differentiate from then branch
                            code.push_str(&self.generate_statement(stmt, nested_index)?);
                        }
                    } else {
                        let nested_index = index * 100 + 2;
                        code.push_str(&self.generate_statement(else_stmt, nested_index)?);
                    }
                }
                
                // End label
                code.push_str(&format!("{}:\n", end_label));
            },
            _ => {
                self.error_handler.report_error(0, &format!("Type d'instruction non pris en charge: {:?}", stmt));
                return Err(0);
            }
        }
        
        Ok(code)
    }
    
    fn generate_expr_code(&mut self, expr: &Expr) -> Result<String, usize> {
        let mut code = String::new();
        
        match expr {
            Expr::Literal(Literal::Int(value)) => {
                code.push_str(&format!("    mov rax, {}\n", value));
            },
            Expr::Literal(Literal::String(_value)) => {
                // Renommé avec underscore pour éviter l'avertissement
                self.error_handler.report_error(0, "Les chaînes littérales ne sont pas encore prises en charge dans les expressions");
                return Err(0);
            },
            Expr::Variable(name) => {
                // Récupérer les informations sur la variable
                if let Some(var_info) = self.variable_info.get(name) {
                    // Charger la variable selon son type, en utilisant des instructions plus sûres
                    match var_info.var_type {
                        Type::I32 => {
                            code.push_str(&format!("    mov eax, DWORD [rbp-{}]  ; Load variable {}\n", var_info.offset, name));
                            code.push_str("    movsx rax, eax  ; Sign extend to 64-bit\n");
                        },
                        Type::I8 => code.push_str(&format!("    movsx rax, BYTE [rbp-{}]\n", var_info.offset)),
                        Type::I16 => code.push_str(&format!("    movsx rax, WORD [rbp-{}]\n", var_info.offset)),
                        Type::I64 | Type::I128 => code.push_str(&format!("    mov rax, QWORD [rbp-{}]\n", var_info.offset)),
                        Type::String => code.push_str(&format!("    mov rax, QWORD [rbp-{}]\n", var_info.offset)),
                        _ => {
                            code.push_str(&format!("    mov eax, DWORD [rbp-{}]  ; Load variable {}\n", var_info.offset, name));
                            code.push_str("    movsx rax, eax  ; Sign extend to 64-bit\n");
                        }
                    }
                } else {
                    // Vérifier si c'est un paramètre de fonction
                    if let Some(current_func) = &self.current_function {
                        if let Some(param_names) = self.function_params.get(current_func) {
                            if let Some(param_index) = param_names.iter().position(|p| p == name) {
                                let offset = (param_index + 1) * 8;
                                code.push_str(&format!("    mov rax, QWORD [rbp-{}]  ; Load parameter {}\n", offset, name));
                                return Ok(code);
                            }
                        }
                    }
                    
                    self.error_handler.report_error(0, &format!("Variable non trouvée: {}", name));
                    code.push_str(&format!("    mov rax, 0  ; Placeholder for variable {}\n", name));
                }
            },
            Expr::Binary(left, op, right) => {
                // Évaluer d'abord l'opérande gauche
                code.push_str(&self.generate_expr_code(left)?);
                // Sauvegarder le résultat
                code.push_str("    push rax\n");
                // Évaluer ensuite l'opérande droite
                code.push_str(&self.generate_expr_code(right)?);
                // Opérande droite est dans rax, gauche dans la pile
                code.push_str("    mov rcx, rax\n");
                code.push_str("    pop rax\n");
                
                // Effectuer l'opération
                match op {
                    BinaryOp::Add => {
                        code.push_str("    add rax, rcx\n");
                    },
                    BinaryOp::Subtract => {
                        code.push_str("    sub rax, rcx\n");
                    },
                    BinaryOp::Multiply => {
                        code.push_str("    imul rax, rcx\n");
                    },
                    BinaryOp::Divide => {
                        code.push_str("    xor rdx, rdx\n");  // Nettoyer rdx pour la division
                        code.push_str("    idiv rcx\n");
                    },
                    // Add comparison operators for if conditions
                    BinaryOp::Equal => {
                        code.push_str("    cmp rax, rcx\n");
                        code.push_str("    sete al\n");      // Set AL to 1 if equal, 0 otherwise
                        code.push_str("    movzx rax, al\n"); // Zero-extend AL to RAX
                    },
                    BinaryOp::NotEqual => {
                        code.push_str("    cmp rax, rcx\n");
                        code.push_str("    setne al\n");     // Set AL to 1 if not equal, 0 otherwise
                        code.push_str("    movzx rax, al\n");
                    },
                    BinaryOp::Less => {
                        code.push_str("    cmp rax, rcx\n");
                        code.push_str("    setl al\n");      // Set AL to 1 if less, 0 otherwise
                        code.push_str("    movzx rax, al\n");
                    },
                    BinaryOp::LessEqual => {
                        code.push_str("    cmp rax, rcx\n");
                        code.push_str("    setle al\n");     // Set AL to 1 if less or equal, 0 otherwise
                        code.push_str("    movzx rax, al\n");
                    },
                    BinaryOp::Greater => {
                        code.push_str("    cmp rax, rcx\n");
                        code.push_str("    setg al\n");      // Set AL to 1 if greater, 0 otherwise
                        code.push_str("    movzx rax, al\n");
                    },
                    BinaryOp::GreaterEqual => {
                        code.push_str("    cmp rax, rcx\n");
                        code.push_str("    setge al\n");     // Set AL to 1 if greater or equal, 0 otherwise
                        code.push_str("    movzx rax, al\n");
                    },
                    _ => {
                        self.error_handler.report_error(0, &format!("Opérateur binaire non pris en charge: {:?}", op));
                        return Err(0);
                    }
                }
            },
            Expr::FunctionCall(callee, args) => {
                code.push_str(&format!("\n    ; Appel de fonction: {}()\n", callee));
                
                // Sauvegarder les registres volatiles avant l'appel
                code.push_str("    ; Sauvegarde des registres volatiles\n");
                code.push_str("    push rcx\n");
                code.push_str("    push rdx\n");
                code.push_str("    push rsi\n");
                code.push_str("    push rdi\n");
                
                // Préparer les arguments dans les registres
                let registers = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                
                // Alignement de la pile (important pour les appels de fonction)
                let stack_adjustment = if args.len() > 6 {
                    (args.len() - 6) * 8
                } else {
                    0
                };
                
                if stack_adjustment > 0 {
                    code.push_str(&format!("    sub rsp, {}\n", stack_adjustment));
                }
                
                // Mettre les arguments dans les registres ou sur la pile
                for (i, arg) in args.iter().enumerate() {
                    code.push_str(&self.generate_expr_code(arg)?);
                    
                    if i < 6 {
                        code.push_str(&format!("    mov {}, rax\n", registers[i]));
                    } else {
                        // Empiler les arguments supplémentaires
                        code.push_str(&format!("    mov QWORD [rsp+{}], rax\n", (i - 6) * 8));
                    }
                }
                
                // Appel de la fonction
                code.push_str(&format!("    call {}\n", callee));
                
                // Restaurer la pile si nécessaire
                if stack_adjustment > 0 {
                    code.push_str(&format!("    add rsp, {}\n", stack_adjustment));
                }
                
                // Restaurer les registres volatiles
                code.push_str("    ; Restauration des registres volatiles\n");
                code.push_str("    pop rdi\n");
                code.push_str("    pop rsi\n");
                code.push_str("    pop rdx\n");
                code.push_str("    pop rcx\n");
                
                // Le résultat est déjà dans rax
            },
            _ => {
                self.error_handler.report_error(0, &format!("Type d'expression non pris en charge: {:?}", expr));
                return Err(0);
            }
        }
        
        Ok(code)
    }
    
    // Remove broken methods and fix the remaining ones
    fn collect_all_strings_in_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let(_, Some(_expr), _, _) => {
                // Process initializer expression if needed
            },
            Stmt::Let(_, None, _, _) => {
                // No initializer, nothing to process
            },
            Stmt::For(_, _range_start, _range_end, body) => {
                // Process loop body
                if let Stmt::Block(stmts) = &**body {
                    for stmt in stmts {
                        self.collect_all_strings_in_stmt(stmt);
                    }
                }
            },
            _ => {
                // Handle other statement types
            }
        }
    }
    
    fn process_function_for_format_labels(&mut self, function: &Function) {
        for (stmt_idx, stmt) in function.body.iter().enumerate() {
            self.process_statement_for_format_labels(stmt, &function.name, stmt_idx);
        }
    }

    fn process_statement_for_format_labels(&mut self, stmt: &Stmt, function_name: &str, index: usize) {
        match stmt {
            Stmt::Println(args) => {
                if !args.is_empty() {
                    if let Expr::Literal(Literal::String(_)) = &args[0] {
                        let label = format!("fmt_{}_{}", function_name, index);
                        self.format_labels.insert((function_name.to_string(), index), label);
                    }
                }
            },
            Stmt::For(_, _, _, body) => {
                if let Stmt::Block(stmts) = &**body {
                    for (i, nested_stmt) in stmts.iter().enumerate() {
                        let nested_index = index * 100 + i + 1;
                        self.process_statement_for_format_labels(nested_stmt, function_name, nested_index);
                    }
                }
            },
            Stmt::If(_, then_stmt, else_stmt) => {
                // Process then branch - handle both Block and non-Block statements
                if let Stmt::Block(stmts) = &**then_stmt {
                    for (i, nested_stmt) in stmts.iter().enumerate() {
                        let nested_index = index * 100 + i + 1;
                        self.process_statement_for_format_labels(nested_stmt, function_name, nested_index);
                    }
                } else {
                    let then_index = index * 100 + 1;
                    self.process_statement_for_format_labels(then_stmt, function_name, then_index);
                }
                
                // Process else branch if it exists - handle both Block and non-Block statements
                if let Some(else_stmt) = else_stmt {
                    if let Stmt::Block(stmts) = &**else_stmt {
                        for (i, nested_stmt) in stmts.iter().enumerate() {
                            let nested_index = index * 100 + i + 2; // Start from +2 to avoid collision with then branch
                            self.process_statement_for_format_labels(nested_stmt, function_name, nested_index);
                        }
                    } else {
                        let else_index = index * 100 + 2;
                        self.process_statement_for_format_labels(else_stmt, function_name, else_index);
                    }
                }
            },
            Stmt::While(_, body) => {
                if let Stmt::Block(stmts) = &**body {
                    for (i, nested_stmt) in stmts.iter().enumerate() {
                        let nested_index = index * 100 + i + 1;
                        self.process_statement_for_format_labels(nested_stmt, function_name, nested_index);
                    }
                } else {
                    let body_index = index * 100 + 1;
                    self.process_statement_for_format_labels(body, function_name, body_index);
                }
            },
            Stmt::Block(stmts) => {
                for (i, nested_stmt) in stmts.iter().enumerate() {
                    let nested_index = index * 100 + i + 1;
                    self.process_statement_for_format_labels(nested_stmt, function_name, nested_index);
                }
            },
            _ => {}
        }
    }

    fn generate_format_strings_for_function(&mut self, function: &Function, code: &mut String) {
        for (stmt_idx, stmt) in function.body.iter().enumerate() {
            self.generate_format_strings_for_statement(stmt, &function.name, stmt_idx, code);
        }
    }

    fn generate_format_strings_for_statement(&mut self, stmt: &Stmt, function_name: &str, index: usize, code: &mut String) {
        match stmt {
            Stmt::Println(args) => {
                if !args.is_empty() {
                    if let Expr::Literal(Literal::String(format_str)) = &args[0] {
                        let c_format = format_str.replace("{}", "%d");
                        if let Some(label) = self.format_labels.get(&(function_name.to_string(), index)) {
                            code.push_str(&format!("    {} db \"{}\", 10, 0\n", label, c_format));
                        }
                    }
                }
            },
            Stmt::For(_, _, _, body) => {
                if let Stmt::Block(stmts) = &**body {
                    for (i, nested_stmt) in stmts.iter().enumerate() {
                        let nested_index = index * 100 + i + 1;
                        self.generate_format_strings_for_statement(nested_stmt, function_name, nested_index, code);
                    }
                }
            },
            Stmt::If(_, then_stmt, else_stmt) => {
                // Process then branch - handle both Block and non-Block statements
                if let Stmt::Block(stmts) = &**then_stmt {
                    for (i, nested_stmt) in stmts.iter().enumerate() {
                        let nested_index = index * 100 + i + 1;
                        self.generate_format_strings_for_statement(nested_stmt, function_name, nested_index, code);
                    }
                } else {
                    let then_index = index * 100 + 1;
                    self.generate_format_strings_for_statement(then_stmt, function_name, then_index, code);
                }
                
                // Process else branch if it exists - handle both Block and non-Block statements
                if let Some(else_stmt) = else_stmt {
                    if let Stmt::Block(stmts) = &**else_stmt {
                        for (i, nested_stmt) in stmts.iter().enumerate() {
                            let nested_index = index * 100 + i + 2; // Start from +2 to avoid collision with then branch
                            self.generate_format_strings_for_statement(nested_stmt, function_name, nested_index, code);
                        }
                    } else {
                        let else_index = index * 100 + 2;
                        self.generate_format_strings_for_statement(else_stmt, function_name, else_index, code);
                    }
                }
            },
            Stmt::While(_, body) => {
                if let Stmt::Block(stmts) = &**body {
                    for (i, nested_stmt) in stmts.iter().enumerate() {
                        let nested_index = index * 100 + i + 1;
                        self.generate_format_strings_for_statement(nested_stmt, function_name, nested_index, code);
                    }
                } else {
                    let body_index = index * 100 + 1;
                    self.generate_format_strings_for_statement(body, function_name, body_index, code);
                }
            },
            Stmt::Block(stmts) => {
                for (i, nested_stmt) in stmts.iter().enumerate() {
                    let nested_index = index * 100 + i + 1;
                    self.generate_format_strings_for_statement(nested_stmt, function_name, nested_index, code);
                }
            },
            _ => {}
        }
    }
    
    // New helper method to count all variables recursively
    fn count_all_variables(&self, statements: &[Stmt], count: &mut usize) {
        for stmt in statements {
            match stmt {
                Stmt::Let(_, _, _, _) => {
                    *count += 1;
                },
                Stmt::For(_, _range_start, _range_end, body) => {
                    *count += 1; // Count the loop variable
                    if let Stmt::Block(stmts) = &**body {
                        self.count_all_variables(stmts, count);
                    }
                },
                Stmt::If(_, then_stmt, else_stmt) => {
                    // Count variables in then branch
                    if let Stmt::Block(stmts) = &**then_stmt {
                        self.count_all_variables(stmts, count);
                    }
                    // Count variables in else branch if it exists
                    if let Some(else_stmt) = else_stmt {
                        if let Stmt::Block(stmts) = &**else_stmt {
                            self.count_all_variables(stmts, count);
                        }
                    }
                },
                Stmt::While(_, body) => {
                    if let Stmt::Block(stmts) = &**body {
                        self.count_all_variables(stmts, count);
                    }
                },
                Stmt::Block(stmts) => {
                    self.count_all_variables(stmts, count);
                },
                _ => {}
            }
        }
    }

    // New helper method to assign offsets to all variables
    fn assign_variable_offsets(&mut self, statements: &[Stmt], offset: &mut usize) {
        for stmt in statements {
            match stmt {
                Stmt::Let(name, _, _, var_type) => {
                    *offset += 8;
                    self.variable_info.insert(name.clone(), VarInfo { 
                        offset: *offset, 
                        var_type: var_type.clone() 
                    });
                },
                Stmt::For(var_name, _, _, body) => {
                    // Assign offset for loop variable if not already assigned
                    if !self.variable_info.contains_key(var_name) {
                        *offset += 8;
                        self.variable_info.insert(var_name.clone(), VarInfo { 
                            offset: *offset, 
                            var_type: Type::I32 
                        });
                    }
                    if let Stmt::Block(stmts) = &**body {
                        self.assign_variable_offsets(stmts, offset);
                    }
                },
                Stmt::If(_, then_stmt, else_stmt) => {
                    // Assign offsets for variables in then branch
                    if let Stmt::Block(stmts) = &**then_stmt {
                        self.assign_variable_offsets(stmts, offset);
                    }
                    // Assign offsets for variables in else branch if it exists
                    if let Some(else_stmt) = else_stmt {
                        if let Stmt::Block(stmts) = &**else_stmt {
                            self.assign_variable_offsets(stmts, offset);
                        }
                    }
                },
                Stmt::While(_, body) => {
                    if let Stmt::Block(stmts) = &**body {
                        self.assign_variable_offsets(stmts, offset);
                    }
                },
                Stmt::Block(stmts) => {
                    self.assign_variable_offsets(stmts, offset);
                },
                _ => {}
            }
        }
    }
}