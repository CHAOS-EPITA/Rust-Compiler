use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

use crate::error_handler::ErrorHandler;
use crate::parser::{Program, Function, Stmt, Expr, BinaryOp, Literal};

pub struct CodeGenerator<'a> {
    error_handler: &'a ErrorHandler,
    current_function: Option<String>,
    label_counter: usize,  // Utile pour générer des étiquettes uniques
    variable_offsets: HashMap<String, usize>,
    function_params: HashMap<String, Vec<String>>,
    format_labels: HashMap<(String, usize), String>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(error_handler: &'a ErrorHandler) -> Self {
        CodeGenerator { 
            error_handler,
            current_function: None,
            label_counter: 0,
            variable_offsets: HashMap::new(),
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
            for (stmt_idx, stmt) in function.body.iter().enumerate() {
                if let Stmt::Println(args) = stmt {
                    if !args.is_empty() {
                        if let Expr::Literal(Literal::String(_)) = &args[0] {
                            let label = format!("fmt_{}_{}",
                                function.name, stmt_idx);
                            self.format_labels.insert((function.name.clone(), stmt_idx), label.clone());
                        }
                    }
                }
            }
        }
        
        // Maintenant générons les définitions de chaînes avec les étiquettes stables
        for function in &program.functions {
            for (stmt_idx, stmt) in function.body.iter().enumerate() {
                if let Stmt::Println(args) = stmt {
                    if !args.is_empty() {
                        if let Expr::Literal(Literal::String(format_str)) = &args[0] {
                            let c_format = format_str.replace("{}", "%d");
                            if let Some(label) = self.format_labels.get(&(function.name.clone(), stmt_idx)) {
                                code.push_str(&format!("    {} db \"{}\", 10, 0\n", label, c_format));
                            }
                        }
                    }
                }
            }
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
                self.variable_offsets.clear();
                code.push_str(&self.generate_function(function)?);
            }
        }
        
        // Traitement spécial pour main pour qu'il suive la convention d'appel C
        if let Some(main_function) = program.functions.iter().find(|f| f.name == "main") {
            self.variable_offsets.clear();
            self.current_function = Some("main".to_string());
            
            code.push_str("main:\n");
            code.push_str("    push rbp\n");
            code.push_str("    mov rbp, rsp\n");
            
            // Allouer de l'espace pour les variables locales
            let local_vars_size = main_function.body.iter()
                .filter(|stmt| matches!(stmt, Stmt::Let(_, _, _)))
                .count() * 8;
            
            if local_vars_size > 0 {
                code.push_str(&format!("    sub rsp, {}\n", local_vars_size));
            }
            
            // Enregistrer les décalages des variables locales
            let mut offset = 0;
            for stmt in &main_function.body {
                if let Stmt::Let(name, _, _) = stmt {
                    offset += 8;
                    self.variable_offsets.insert(name.clone(), offset);
                }
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
        self.variable_offsets.clear();
        
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
                .filter(|stmt| matches!(stmt, Stmt::Let(_, _, _)))
                .count();
            
            let stack_size = (total_local_vars + param_names.len()) * 8;
            if stack_size > 0 {
                code.push_str(&format!("    sub rsp, {}\n", stack_size));
            }
            
            // Sauvegarder les paramètres sur la pile
            for (i, param_name) in param_names.iter().enumerate() {
                let offset = (i + 1) * 8;
                self.variable_offsets.insert(param_name.clone(), offset);
                
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
                if let Stmt::Let(name, _, _) = stmt {
                    current_offset += 8;
                    self.variable_offsets.insert(name.clone(), current_offset);
                }
            }
        } else {
            // Pas de paramètres, juste gérer les variables locales
            let local_vars_size = function.body.iter()
                .filter(|stmt| matches!(stmt, Stmt::Let(_, _, _)))
                .count() * 8;
            
            if local_vars_size > 0 {
                code.push_str(&format!("    sub rsp, {}\n", local_vars_size));
            }
            
            // Enregistrer les décalages des variables locales
            let mut offset = 0;
            for stmt in &function.body {
                if let Stmt::Let(name, _, _) = stmt {
                    offset += 8;
                    self.variable_offsets.insert(name.clone(), offset);
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
            Stmt::Let(name, initializer, _mutable) => {
                let offset = self.variable_offsets.get(name).cloned().unwrap_or_else(|| {
                    self.error_handler.report_error(0, &format!("Offset de variable introuvable pour: {}", name));
                    8 * (index + 1)
                });
                
                code.push_str(&format!("\n    ; Variable declaration: {}\n", name));
                
                if let Some(init_expr) = initializer {
                    // Évaluer l'expression et la stocker dans rax
                    code.push_str(&self.generate_expr_code(init_expr)?);
                    // Stocker la valeur à l'emplacement approprié
                    code.push_str(&format!("    mov QWORD [rbp-{}], rax\n", offset));
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
                    for (i, arg) in args.iter().skip(1).enumerate() {
                        code.push_str(&format!("\n    ; Évaluation de l'argument {}\n", i + 1));
                        code.push_str(&self.generate_expr_code(arg)?);
                        code.push_str("    push rax  ; Sauvegarde de l'argument sur la pile\n");
                    }
                    
                    // Maintenant, charger tous les arguments dans les registres dans l'ordre inverse
                    let registers = ["r9", "r8", "rcx", "rdx", "rsi"]; // Ordre inverse
                    code.push_str("\n    ; Configuration des registres pour printf\n");
                    
                    // Récupérer les arguments de la pile dans l'ordre inverse
                    for i in 0..std::cmp::min(args.len() - 1, registers.len()) {
                        let reg_index = args.len() - 2 - i; // Ordre inverse pour la pile
                        code.push_str(&format!("    pop {}  ; Récupération de l'argument {}\n", 
                            registers[reg_index], args.len() - i - 1));
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
                // Récupérer le décalage réel de la variable
                if let Some(offset) = self.variable_offsets.get(name) {
                    code.push_str(&format!("    mov rax, QWORD [rbp-{}]  ; Load variable {}\n", offset, name));
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
}