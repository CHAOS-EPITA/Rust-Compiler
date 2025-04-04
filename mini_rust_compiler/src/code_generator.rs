use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::error_handler::ErrorHandler;
use crate::parser::{Program, Function, Stmt, Expr, BinaryOp, UnaryOp, Literal};

pub struct CodeGenerator<'a> {
    error_handler: &'a ErrorHandler,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(error_handler: &'a ErrorHandler) -> Self {
        CodeGenerator { error_handler }
    }
    
    pub fn generate(&mut self, program: Program, source_path: &str) -> Result<String, usize> {
        // Générer le code C pour le programme
        let c_code = self.generate_c_code(&program)?;
        
        // Déterminer le nom de l'exécutable
        let source_path = Path::new(source_path);
        let stem = if let Some(stem) = source_path.file_stem() {
            stem.to_str().unwrap().to_string()
        } else {
            "a.out".to_string()
        };
        
        // Créer un fichier C intermédiaire
        let c_file_path = format!("{}.c", stem);
        
        let mut c_file = match File::create(&c_file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(0);
            }
        };
        
        if let Err(e) = c_file.write_all(c_code.as_bytes()) {
            return Err(0);
        }
        
        // Compiler le code C en un exécutable
        let gcc_command = format!("gcc -o {} {}", stem, c_file_path);
        
        let output = match Command::new("gcc")
            .arg("-o")
            .arg(&stem)
            .arg(&c_file_path)
            .output() {
                Ok(output) => output,
                Err(e) => {
                    return Err(0);
                }
            };
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Erreur de compilation : {}", error);
            return Err(0);
        }
    
        
        // Optionnel : Suppression du fichier C intermédiaire
        // std::fs::remove_file(&c_file_path).ok();
        
        Ok(stem)
    }
    
    fn generate_c_code(&self, program: &Program) -> Result<String, usize> {
        let mut code = String::new();
        
        // En-têtes
        code.push_str("#include <stdio.h>\n");
        code.push_str("#include <stdlib.h>\n\n");
        
        // Déclarations de fonctions
        for function in &program.functions {
            code.push_str(&self.generate_function_prototype(function)?);
            code.push_str(";\n");
        }
        
        code.push_str("\n");
        
        // Implémentation des fonctions
        for function in &program.functions {
            code.push_str(&self.generate_function(function)?);
            code.push_str("\n\n");
        }
        
        Ok(code)
    }
    
    fn generate_function_prototype(&self, function: &Function) -> Result<String, usize> {
        let mut prototype = String::new();
        
        // Type de retour
        if let Some(return_type) = &function.return_type {
            if return_type == "i32" {
                prototype.push_str("int ");
            } else {
                return Err(0);
            }
        } else {
            prototype.push_str("void ");
        }
        
        // Nom de la fonction
        prototype.push_str(&function.name);
        
        // Paramètres
        prototype.push_str("(");
        if !function.params.is_empty() {
            for (i, (name, param_type)) in function.params.iter().enumerate() {
                if param_type == "i32" {
                    prototype.push_str(&format!("int {}", name));
                } else {
                    return Err(0);
                }
                
                if i < function.params.len() - 1 {
                    prototype.push_str(", ");
                }
            }
        } else {
            prototype.push_str("void");
        }
        prototype.push_str(")");
        
        Ok(prototype)
    }
    
    fn generate_function(&self, function: &Function) -> Result<String, usize> {
        let mut code = String::new();
        
        // Prototype de la fonction
        code.push_str(&self.generate_function_prototype(function)?);
        
        // Corps de la fonction
        code.push_str(" {\n");
        
        // Générer le code pour chaque instruction
        for stmt in &function.body {
            code.push_str(&self.generate_statement(stmt, 1)?);
        }
        
        code.push_str("}\n");
        
        Ok(code)
    }
    
    fn generate_statement(&self, stmt: &Stmt, indent: usize) -> Result<String, usize> {
        let indent_str = "    ".repeat(indent);
        let mut code = String::new();
        
        match stmt {
            Stmt::Expression(expr) => {
                code.push_str(&indent_str);
                code.push_str(&self.generate_expression(expr)?);
                code.push_str(";\n");
            },
            Stmt::Let(name, initializer, _mutable) => {
                code.push_str(&indent_str);
                code.push_str("int ");
                code.push_str(name);
                
                if let Some(init_expr) = initializer {
                    code.push_str(" = ");
                    code.push_str(&self.generate_expression(init_expr)?);
                }
                
                code.push_str(";\n");
            },
            Stmt::Assign(name, expr) => {
                code.push_str(&indent_str);
                code.push_str(name);
                code.push_str(" = ");
                code.push_str(&self.generate_expression(expr)?);
                code.push_str(";\n");
            },
            Stmt::Block(statements) => {
                code.push_str(&indent_str);
                code.push_str("{\n");
                
                for stmt in statements {
                    code.push_str(&self.generate_statement(stmt, indent + 1)?);
                }
                
                code.push_str(&indent_str);
                code.push_str("}\n");
            },
            Stmt::If(condition, then_branch, else_branch) => {
                code.push_str(&indent_str);
                code.push_str("if (");
                code.push_str(&self.generate_expression(condition)?);
                code.push_str(") ");
                
                match &**then_branch {
                    Stmt::Block(_) => code.push_str(&self.generate_statement(then_branch, indent)?),
                    _ => {
                        code.push_str("{\n");
                        code.push_str(&self.generate_statement(then_branch, indent + 1)?);
                        code.push_str(&indent_str);
                        code.push_str("}\n");
                    }
                }
                
                if let Some(else_stmt) = else_branch {
                    code.push_str(&indent_str);
                    code.push_str("else ");
                    
                    match &**else_stmt {
                        Stmt::Block(_) | Stmt::If(_, _, _) => {
                            code.push_str(&self.generate_statement(else_stmt, indent)?);
                        },
                        _ => {
                            code.push_str("{\n");
                            code.push_str(&self.generate_statement(else_stmt, indent + 1)?);
                            code.push_str(&indent_str);
                            code.push_str("}\n");
                        }
                    }
                }
            },
            Stmt::While(condition, body) => {
                code.push_str(&indent_str);
                code.push_str("while (");
                code.push_str(&self.generate_expression(condition)?);
                code.push_str(") ");
                
                match &**body {
                    Stmt::Block(_) => code.push_str(&self.generate_statement(body, indent)?),
                    _ => {
                        code.push_str("{\n");
                        code.push_str(&self.generate_statement(body, indent + 1)?);
                        code.push_str(&indent_str);
                        code.push_str("}\n");
                    }
                }
            },
            Stmt::Return(expr) => {
                code.push_str(&indent_str);
                code.push_str("return");
                
                if let Some(ret_expr) = expr {
                    code.push_str(" ");
                    code.push_str(&self.generate_expression(ret_expr)?);
                }
                
                code.push_str(";\n");
            },
            Stmt::Println(args) => {
                // Version simplifiée de println! qui ne gère que les chaînes de format basiques
                if args.is_empty() {
                    code.push_str(&indent_str);
                    code.push_str("printf(\"\\n\");\n");
                } else if let Expr::Literal(Literal::String(format_str)) = &args[0] {
                    // Convertir les {} en %d pour les entiers
                    let c_format = format_str.replace("{}", "%d");
                    
                    code.push_str(&indent_str);
                    code.push_str("printf(\"");
                    code.push_str(&c_format);
                    code.push_str("\\n\"");
                    
                    // Ajouter les arguments
                    for arg in args.iter().skip(1) {
                        code.push_str(", ");
                        code.push_str(&self.generate_expression(arg)?);
                    }
                    
                    code.push_str(");\n");
                } else {
                    return Err(0);
                }
            },
        }
        
        Ok(code)
    }
    
    fn generate_expression(&self, expr: &Expr) -> Result<String, usize> {
        match expr {
            Expr::Binary(left, op, right) => {
                let left_code = self.generate_expression(left)?;
                let right_code = self.generate_expression(right)?;
                
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Subtract => "-",
                    BinaryOp::Multiply => "*",
                    BinaryOp::Divide => "/",
                    BinaryOp::Modulo => "%",
                    BinaryOp::Equal => "==",
                    BinaryOp::NotEqual => "!=",
                    BinaryOp::Less => "<",
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::Greater => ">",
                    BinaryOp::GreaterEqual => ">=",
                };
                
                Ok(format!("({} {} {})", left_code, op_str, right_code))
            },
            Expr::Unary(op, expr) => {
                let expr_code = self.generate_expression(expr)?;
                
                let op_str = match op {
                    UnaryOp::Negate => "-",
                };
                
                Ok(format!("({}{})", op_str, expr_code))
            },
            Expr::Literal(literal) => {
                match literal {
                    Literal::Int(value) => Ok(format!("{}", value)),
                    Literal::String(value) => Ok(format!("\"{}\"", value)),
                }
            },
            Expr::Variable(name) => Ok(name.clone()),
            Expr::FunctionCall(callee, args) => {
                let mut code = String::new();
                
                code.push_str(callee);
                code.push_str("(");
                
                for (i, arg) in args.iter().enumerate() {
                    code.push_str(&self.generate_expression(arg)?);
                    
                    if i < args.len() - 1 {
                        code.push_str(", ");
                    }
                }
                
                code.push_str(")");
                
                Ok(code)
            },
            Expr::Call(_, _) => Err(0), // Non implémenté
        }
    }
}