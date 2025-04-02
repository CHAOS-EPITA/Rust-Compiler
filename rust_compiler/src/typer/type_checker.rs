use crate::common::{CompilerError, Result};
use crate::parser::ast::*;
use super::types::{Type, TypeEnvironment, from_ast_type};

/// TypeChecker vérifie les types dans un programme Rust
pub struct TypeChecker {
    env: TypeEnvironment,
    source: String,
}

impl TypeChecker {
    /// Crée un nouveau vérificateur de types
    pub fn new(source: String) -> Self {
        Self {
            env: TypeEnvironment::new(),
            source,
        }
    }

    /// Vérifie les types d'un programme complet
    pub fn check_program(&mut self, program: &Program) -> Result<()> {
        // Première passe: enregistrer les déclarations de fonctions
        for decl in &program.declarations {
            if let Declaration::Function(func) = decl {
                self.register_function(func)?;
            }
        }

        // Deuxième passe: vérifier toutes les déclarations
        for decl in &program.declarations {
            self.check_declaration(decl)?;
        }

        Ok(())
    }

    /// Enregistre une fonction dans l'environnement de types
    fn register_function(&mut self, func: &FunctionDecl) -> Result<()> {
        let mut param_types = Vec::new();

        for param in &func.params {
            param_types.push(from_ast_type(&param.typ));
        }

        let return_type = if let Some(rt) = &func.return_type {
            from_ast_type(rt)
        } else {
            Type::Unit
        };

        let function_type = Type::Function {
            params: param_types,
            return_type: Box::new(return_type),
        };

        self.env.define(func.name.clone(), function_type);
        Ok(())
    }

    /// Vérifie une déclaration
    fn check_declaration(&mut self, decl: &Declaration) -> Result<()> {
        match decl {
            Declaration::Function(func) => self.check_function(func),
            Declaration::Variable(var) => self.check_variable_decl(var),
        }
    }

    /// Vérifie une déclaration de fonction
    fn check_function(&mut self, func: &FunctionDecl) -> Result<()> {
        self.env.enter_scope();

        // Ajouter les paramètres dans le scope
        for param in &func.params {
            let typ = from_ast_type(&param.typ);
            self.env.define(param.name.clone(), typ);
        }

        // Vérifier le corps de la fonction
        let expected_return_type = if let Some(rt) = &func.return_type {
            from_ast_type(rt)
        } else {
            Type::Unit
        };

        self.check_block(&func.body, Some(&expected_return_type))?;

        self.env.exit_scope();
        Ok(())
    }

    /// Vérifie une déclaration de variable
    fn check_variable_decl(&mut self, var: &VariableDecl) -> Result<()> {
        let expr_type = self.check_expr(&var.initializer)?;

        if let Some(declared_type) = &var.typ {
            let expected_type = from_ast_type(declared_type);
            if expr_type != expected_type {
                return Err(CompilerError::TypeError {
                    src: self.source.clone(),
                    span: var.span.into(),
                    message: format!(
                        "Type mismatch: expected {}, found {}",
                        expected_type, expr_type
                    ),
                });
            }
        }

        // Définir la variable dans l'environnement
        self.env.define(var.name.clone(), expr_type);
        Ok(())
    }

    /// Vérifie un bloc de code
    fn check_block(&mut self, block: &Block, expected_return_type: Option<&Type>) -> Result<Type> {
        self.env.enter_scope();
        
        let mut last_type = Type::Unit;
        
        for stmt in &block.statements {
            last_type = self.check_stmt(stmt, expected_return_type)?;
        }
        
        self.env.exit_scope();
        Ok(last_type)
    }

    /// Vérifie une instruction
    fn check_stmt(&mut self, stmt: &Stmt, expected_return_type: Option<&Type>) -> Result<Type> {
        match stmt {
            Stmt::Expr(expr_stmt) => self.check_expr(&expr_stmt.expr),
            Stmt::Declaration(decl) => {
                self.check_declaration(decl)?;
                Ok(Type::Unit)
            }
            Stmt::If(if_stmt) => self.check_if_stmt(if_stmt, expected_return_type),
            Stmt::While(while_stmt) => self.check_while_stmt(while_stmt),
            Stmt::For(for_stmt) => self.check_for_stmt(for_stmt),
            Stmt::Return(return_stmt) => self.check_return_stmt(return_stmt, expected_return_type),
            Stmt::Block(block) => self.check_block(block, expected_return_type),
        }
    }

    /// Vérifie une instruction if-else
    fn check_if_stmt(&mut self, if_stmt: &IfStmt, expected_return_type: Option<&Type>) -> Result<Type> {
        let cond_type = self.check_expr(&if_stmt.condition)?;
        
        if cond_type != Type::Bool {
            return Err(CompilerError::TypeError {
                src: self.source.clone(),
                span: if_stmt.condition.span().into(),
                message: format!("If condition must be a boolean, found {}", cond_type),
            });
        }
        
        let then_type = self.check_block(&if_stmt.then_branch, expected_return_type)?;
        
        if let Some(else_branch) = &if_stmt.else_branch {
            let else_type = self.check_stmt(else_branch, expected_return_type)?;
            
            if then_type != else_type {
                return Err(CompilerError::TypeError {
                    src: self.source.clone(),
                    span: if_stmt.span.into(),
                    message: format!(
                        "If and else branches have different types: {} and {}",
                        then_type, else_type
                    ),
                });
            }
            
            Ok(then_type)
        } else {
            Ok(Type::Unit)
        }
    }

    /// Vérifie une instruction while
    fn check_while_stmt(&mut self, while_stmt: &WhileStmt) -> Result<Type> {
        let cond_type = self.check_expr(&while_stmt.condition)?;
        
        if cond_type != Type::Bool {
            return Err(CompilerError::TypeError {
                src: self.source.clone(),
                span: while_stmt.condition.span().into(),
                message: format!("While condition must be a boolean, found {}", cond_type),
            });
        }
        
        self.check_block(&while_stmt.body, None)?;
        Ok(Type::Unit)
    }

    /// Vérifie une instruction for
    fn check_for_stmt(&mut self, for_stmt: &ForStmt) -> Result<Type> {
        let _iter_type = self.check_expr(&for_stmt.iterator)?;
        
        // TODO: Vérifier que l'itérateur est de type itérable
        // Pour l'instant, on considère que tout est itérable
        
        self.env.enter_scope();
        self.env.define(for_stmt.variable.clone(), Type::Int); // Par simplification
        
        self.check_block(&for_stmt.body, None)?;
        
        self.env.exit_scope();
        Ok(Type::Unit)
    }

    /// Vérifie une instruction return
    fn check_return_stmt(&mut self, return_stmt: &ReturnStmt, expected_type: Option<&Type>) -> Result<Type> {
        let return_type = if let Some(expr) = &return_stmt.value {
            self.check_expr(expr)?
        } else {
            Type::Unit
        };
        
        if let Some(expected) = expected_type {
            if &return_type != expected {
                return Err(CompilerError::TypeError {
                    src: self.source.clone(),
                    span: return_stmt.span.into(),
                    message: format!(
                        "Return type mismatch: expected {}, found {}",
                        expected, return_type
                    ),
                });
            }
        }
        
        Ok(return_type)
    }

    /// Vérifie une expression
    fn check_expr(&mut self, expr: &Expr) -> Result<Type> {
        match expr {
            Expr::Binary(bin_expr) => self.check_binary_expr(bin_expr),
            Expr::Unary(unary_expr) => self.check_unary_expr(unary_expr),
            Expr::Assign(assign_expr) => self.check_assign_expr(assign_expr),
            Expr::Literal(lit) => self.check_literal(lit),
            Expr::Identifier(ident) => self.check_identifier(ident),
            Expr::Call(call) => self.check_function_call(call),
        }
    }

    /// Vérifie une expression binaire
    fn check_binary_expr(&mut self, bin_expr: &BinaryExpr) -> Result<Type> {
        let left_type = self.check_expr(&bin_expr.left)?;
        let right_type = self.check_expr(&bin_expr.right)?;
        
        match bin_expr.op {
            // Opérateurs arithmétiques
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                if left_type == Type::Int && right_type == Type::Int {
                    Ok(Type::Int)
                } else if (left_type == Type::Float && right_type == Type::Float) ||
                          (left_type == Type::Float && right_type == Type::Int) ||
                          (left_type == Type::Int && right_type == Type::Float) {
                    Ok(Type::Float)
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: bin_expr.span.into(),
                        message: format!(
                            "Cannot apply operator {:?} to types {} and {}",
                            bin_expr.op, left_type, right_type
                        ),
                    })
                }
            }
            
            // Opérateurs de comparaison
            BinaryOp::Eq | BinaryOp::NotEq => {
                if left_type == right_type {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: bin_expr.span.into(),
                        message: format!(
                            "Cannot compare types {} and {}",
                            left_type, right_type
                        ),
                    })
                }
            }
            
            BinaryOp::Lt | BinaryOp::LtEq | BinaryOp::Gt | BinaryOp::GtEq => {
                if (left_type == Type::Int && right_type == Type::Int) ||
                   (left_type == Type::Float && right_type == Type::Float) {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: bin_expr.span.into(),
                        message: format!(
                            "Cannot compare types {} and {} with operator {:?}",
                            left_type, right_type, bin_expr.op
                        ),
                    })
                }
            }
            
            // Opérateurs logiques
            BinaryOp::And | BinaryOp::Or => {
                if left_type == Type::Bool && right_type == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: bin_expr.span.into(),
                        message: format!(
                            "Cannot apply logical operator {:?} to types {} and {}",
                            bin_expr.op, left_type, right_type
                        ),
                    })
                }
            }
        }
    }

    /// Vérifie une expression unaire
    fn check_unary_expr(&mut self, unary_expr: &UnaryExpr) -> Result<Type> {
        let operand_type = self.check_expr(&unary_expr.expr)?;
        
        match unary_expr.op {
            UnaryOp::Neg => {
                if operand_type == Type::Int || operand_type == Type::Float {
                    Ok(operand_type)
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: unary_expr.span.into(),
                        message: format!(
                            "Cannot negate value of type {}",
                            operand_type
                        ),
                    })
                }
            }
            
            UnaryOp::Not => {
                if operand_type == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: unary_expr.span.into(),
                        message: format!(
                            "Cannot apply logical not to value of type {}",
                            operand_type
                        ),
                    })
                }
            }
            
            _ => {
                if operand_type == Type::Int || operand_type == Type::Float {
                    Ok(operand_type)
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: unary_expr.span.into(),
                        message: format!(
                            "Cannot apply unary operator to value of type {}",
                            operand_type
                        ),
                    })
                }
            }
        }
    }

    /// Vérifie une expression d'affectation
    fn check_assign_expr(&mut self, assign_expr: &AssignExpr) -> Result<Type> {
        let target_type = if let Expr::Identifier(ident) = &*assign_expr.target {
            if let Some(typ) = self.env.lookup(&ident.name) {
                typ.clone()
            } else {
                return Err(CompilerError::TypeError {
                    src: self.source.clone(),
                    span: ident.span.into(),
                    message: format!("Undefined variable: {}", ident.name),
                });
            }
        } else {
            return Err(CompilerError::TypeError {
                src: self.source.clone(),
                span: assign_expr.target.span().into(),
                message: "Left side of assignment must be a variable".to_string(),
            });
        };
        
        let value_type = self.check_expr(&assign_expr.value)?;
        
        if target_type != value_type {
            return Err(CompilerError::TypeError {
                src: self.source.clone(),
                span: assign_expr.span.into(),
                message: format!(
                    "Cannot assign value of type {} to variable of type {}",
                    value_type, target_type
                ),
            });
        }
        
        Ok(value_type)
    }

    /// Vérifie un littéral
    fn check_literal(&self, lit: &Literal) -> Result<Type> {
        match lit {
            Literal::Int(_, _) => Ok(Type::Int),
            Literal::Float(_, _) => Ok(Type::Float),
            Literal::Bool(_, _) => Ok(Type::Bool),
            Literal::String(_, _) => Ok(Type::String),
        }
    }

    /// Vérifie un identifiant
    fn check_identifier(&self, ident: &Identifier) -> Result<Type> {
        if let Some(typ) = self.env.lookup(&ident.name) {
            Ok(typ.clone())
        } else {
            Err(CompilerError::TypeError {
                src: self.source.clone(),
                span: ident.span.into(),
                message: format!("Undefined variable: {}", ident.name),
            })
        }
    }

    /// Vérifie un appel de fonction
    fn check_function_call(&mut self, call: &FunctionCall) -> Result<Type> {
        if let Expr::Identifier(func_ident) = &*call.callee {
            if let Some(func_type) = self.env.lookup(&func_ident.name) {
                if let Type::Function { params, return_type } = func_type {
                    // Vérifier le nombre d'arguments
                    if params.len() != call.arguments.len() {
                        return Err(CompilerError::TypeError {
                            src: self.source.clone(),
                            span: call.span.into(),
                            message: format!(
                                "Function {} takes {} arguments but {} were provided",
                                func_ident.name, params.len(), call.arguments.len()
                            ),
                        });
                    }
                    
                    // Vérifier les types des arguments
                    for (i, (arg, param_type)) in call.arguments.iter().zip(params.iter()).enumerate() {
                        let arg_type = self.check_expr(arg)?;
                        if arg_type != *param_type {
                            return Err(CompilerError::TypeError {
                                src: self.source.clone(),
                                span: arg.span().into(),
                                message: format!(
                                    "Type mismatch in argument {}: expected {}, found {}",
                                    i, param_type, arg_type
                                ),
                            });
                        }
                    }
                    
                    Ok(*return_type.clone())
                } else {
                    Err(CompilerError::TypeError {
                        src: self.source.clone(),
                        span: func_ident.span.into(),
                        message: format!("{} is not a function", func_ident.name),
                    })
                }
            } else {
                Err(CompilerError::TypeError {
                    src: self.source.clone(),
                    span: func_ident.span.into(),
                    message: format!("Undefined function: {}", func_ident.name),
                })
            }
        } else {
            Err(CompilerError::TypeError {
                src: self.source.clone(),
                span: call.callee.span().into(),
                message: "Callee must be a function name".to_string(),
            })
        }
    }
}
