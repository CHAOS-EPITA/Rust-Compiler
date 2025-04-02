use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use crate::common::{CompilerError, Result, Span};
use super::ast::*;

#[derive(Parser)]
#[grammar = "parser/rust.pest"]
pub struct RustParser;

pub struct AstParser<'a> {
    source: &'a str,
}

impl<'a> AstParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn parse(&self) -> Result<Program> {
        let pairs = RustParser::parse(Rule::program, self.source)
            .map_err(|e| CompilerError::ParserError {
                src: self.source.to_string(),
                span: Span::new(0, self.source.len()).into(),
                message: e.to_string(),
            })?;

        // Convertir les paires en AST
        let mut declarations = Vec::new();
        
        // Trouver la paire programme (premier niveau)
        for pair in pairs {
            if pair.as_rule() == Rule::program {
                // Analyser chaque déclaration dans le programme
                for inner_pair in pair.into_inner() {
                    if inner_pair.as_rule() == Rule::declaration {
                        declarations.push(self.parse_declaration(inner_pair)?);
                    }
                }
                break;
            }
        }
        
        Ok(Program {
            declarations,
            span: Span::new(0, self.source.len()).into(),
        })
    }

    fn parse_declaration(&self, pair: Pair<Rule>) -> Result<Declaration> {
        let inner = pair.into_inner().next().unwrap();
        
        match inner.as_rule() {
            Rule::fn_decl => {
                let function = self.parse_function_decl(inner)?;
                Ok(Declaration::Function(function))
            },
            Rule::let_decl => {
                let variable = self.parse_variable_decl(inner)?;
                Ok(Declaration::Variable(variable))
            },
            _ => {
                Err(CompilerError::ParserError {
                    src: self.source.to_string(),
                    span: self.get_span(&inner).into(),
                    message: format!("Règle de déclaration inattendue: {:?}", inner.as_rule()),
                })
            }
        }
    }

    fn parse_function_decl(&self, pair: Pair<Rule>) -> Result<FunctionDecl> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        // Obtenir le nom de la fonction
        let name_pair = inner_rules.next().unwrap();
        let name = name_pair.as_str().to_string();
        
        // Analyser les paramètres
        let mut params = Vec::new();
        let maybe_param_list = inner_rules.next().unwrap();
        
        if maybe_param_list.as_rule() == Rule::param_list {
            for param_pair in maybe_param_list.into_inner() {
                params.push(self.parse_parameter(param_pair)?);
            }
            
            // Passer au type de retour s'il existe
            let maybe_return_type = inner_rules.next().unwrap();
            
            // Traiter le type de retour si présent
            let return_type = if maybe_return_type.as_rule() == Rule::r#type {
                Some(self.parse_type(&maybe_return_type)?)
            } else {
                None
            };
            
            // Obtenir le bloc
            let block_pair = if return_type.is_some() {
                inner_rules.next().unwrap()
            } else {
                maybe_return_type
            };
            
            let body = self.parse_block(block_pair)?;
            
            Ok(FunctionDecl {
                name,
                params,
                return_type,
                body,
                span: span.into(),
            })
        } else {
            // Pas de paramètres, vérifier le type de retour
            let maybe_return_type = maybe_param_list;
            
            // Traiter le type de retour si présent
            let return_type = if maybe_return_type.as_rule() == Rule::r#type {
                Some(self.parse_type(&maybe_return_type)?)
            } else {
                None
            };
            
            // Obtenir le bloc
            let block_pair = if return_type.is_some() {
                inner_rules.next().unwrap()
            } else {
                maybe_return_type
            };
            
            let body = self.parse_block(block_pair)?;
            
            Ok(FunctionDecl {
                name,
                params,
                return_type,
                body,
                span: span.into(),
            })
        }
    }

    fn parse_parameter(&self, pair: Pair<Rule>) -> Result<Parameter> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        let name_pair = inner_rules.next().unwrap();
        let name = name_pair.as_str().to_string();
        
        let type_pair = inner_rules.next().unwrap();
        let typ = self.parse_type(&type_pair)?;
        
        Ok(Parameter { name, typ, span: span.into() })
    }

    fn parse_variable_decl(&self, pair: Pair<Rule>) -> Result<VariableDecl> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        // Vérifier si mutable
        let first = inner_rules.next().unwrap();
        let (is_mutable, name_pair) = if first.as_rule() == Rule::ident {
            (false, first)
        } else {
            // C'est le token "mut"
            (true, inner_rules.next().unwrap())
        };
        
        let name = name_pair.as_str().to_string();
        
        // Vérifier s'il y a une annotation de type
        let next = inner_rules.next().unwrap();
        let (typ, expr_pair) = if next.as_rule() == Rule::r#type {
            (Some(self.parse_type(&next)?), inner_rules.next().unwrap())
        } else {
            (None, next)
        };
        
        let initializer = self.parse_expr(expr_pair)?;
        
        Ok(VariableDecl {
            name,
            typ,
            is_mutable,
            initializer,
            span: span.into(),
        })
    }

    fn parse_type(&self, pair: &Pair<Rule>) -> Result<Type> {
        match pair.as_str() {
            "i32" => Ok(Type::I32),
            "f64" => Ok(Type::F64),
            "bool" => Ok(Type::Bool),
            "str" => Ok(Type::Str),
            _ => Err(CompilerError::ParserError {
                src: self.source.to_string(),
                span: self.get_span(pair).into(),
                message: format!("Type inconnu: {}", pair.as_str()),
            }),
        }
    }

    fn parse_block(&self, pair: Pair<Rule>) -> Result<Block> {
        let span = self.get_span(&pair);
        let mut statements = Vec::new();
        
        for stmt_pair in pair.into_inner() {
            statements.push(self.parse_stmt(stmt_pair)?);
        }
        
        Ok(Block { statements, span: span.into() })
    }

    fn parse_stmt(&self, pair: Pair<Rule>) -> Result<Stmt> {
        match pair.as_rule() {
            Rule::expr => {
                let span = self.get_span(&pair);
                let expr = self.parse_expr(pair)?;
                Ok(Stmt::Expr(ExprStmt { expr, span: span.into() }))
            },
            Rule::declaration => {
                let decl = self.parse_declaration(pair)?;
                Ok(Stmt::Declaration(decl))
            },
            Rule::if_stmt => self.parse_if_stmt(pair),
            Rule::while_stmt => self.parse_while_stmt(pair),
            Rule::for_stmt => self.parse_for_stmt(pair),
            Rule::return_stmt => self.parse_return_stmt(pair),
            Rule::block => {
                let block = self.parse_block(pair)?;
                Ok(Stmt::Block(block))
            },
            _ => Err(CompilerError::ParserError {
                src: self.source.to_string(),
                span: self.get_span(&pair).into(),
                message: format!("Règle d'instruction inattendue: {:?}", pair.as_rule()),
            }),
        }
    }

    fn parse_if_stmt(&self, pair: Pair<Rule>) -> Result<Stmt> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        let condition_pair = inner_rules.next().unwrap();
        let condition = self.parse_expr(condition_pair)?;
        
        let then_block_pair = inner_rules.next().unwrap();
        let then_branch = self.parse_block(then_block_pair)?;
        
        let else_branch = if let Some(else_pair) = inner_rules.next() {
            // C'est soit un autre if, soit un bloc
            match else_pair.as_rule() {
                Rule::if_stmt => {
                    let else_if = self.parse_if_stmt(else_pair)?;
                    Some(Box::new(else_if))
                },
                Rule::block => {
                    let else_block = self.parse_block(else_pair)?;
                    Some(Box::new(Stmt::Block(else_block)))
                },
                _ => return Err(CompilerError::ParserError {
                    src: self.source.to_string(),
                    span: self.get_span(&else_pair).into(),
                    message: "Branche else inattendue".into(),
                }),
            }
        } else {
            None
        };
        
        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
            span: span.into(),
        }))
    }

    fn parse_while_stmt(&self, pair: Pair<Rule>) -> Result<Stmt> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        let condition_pair = inner_rules.next().unwrap();
        let condition = self.parse_expr(condition_pair)?;
        
        let body_pair = inner_rules.next().unwrap();
        let body = self.parse_block(body_pair)?;
        
        Ok(Stmt::While(WhileStmt { condition, body, span: span.into() }))
    }

    fn parse_for_stmt(&self, pair: Pair<Rule>) -> Result<Stmt> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        let var_pair = inner_rules.next().unwrap();
        let variable = var_pair.as_str().to_string();
        
        let iterator_pair = inner_rules.next().unwrap();
        let iterator = self.parse_expr(iterator_pair)?;
        
        let body_pair = inner_rules.next().unwrap();
        let body = self.parse_block(body_pair)?;
        
        Ok(Stmt::For(ForStmt {
            variable,
            iterator,
            body,
            span: span.into(),
        }))
    }

    fn parse_return_stmt(&self, pair: Pair<Rule>) -> Result<Stmt> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        let value = if let Some(expr_pair) = inner_rules.next() {
            Some(self.parse_expr(expr_pair)?)
        } else {
            None
        };
        
        Ok(Stmt::Return(ReturnStmt { value, span: span.into() }))
    }

    fn parse_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        match pair.as_rule() {
            Rule::expr => {
                // Déléguer au premier enfant
                let inner = pair.into_inner().next().unwrap();
                self.parse_expr(inner)
            },
            Rule::assign_expr => self.parse_assign_expr(pair),
            Rule::logical_or_expr => self.parse_logical_or_expr(pair),
            _ => Err(CompilerError::ParserError {
                src: self.source.to_string(),
                span: self.get_span(&pair).into(),
                message: format!("Règle d'expression inattendue: {:?}", pair.as_rule()),
            }),
        }
    }

    fn parse_assign_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        let target_pair = inner_rules.next().unwrap();
        let target = target_pair.as_str().to_string();
        
        let value_pair = inner_rules.next().unwrap();
        let value = Box::new(self.parse_expr(value_pair)?);
        
        Ok(Expr::Assign(AssignExpr { target, value, span: span.into() }))
    }

    fn parse_logical_or_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        let pairs: Vec<_> = pair.into_inner().collect();
        
        if pairs.len() == 1 {
            // Pas d'opérateur OR, déléguer
            return self.parse_logical_and_expr(pairs[0].clone());
        }
        
        // Construire une expression binaire pour chaque opérateur OR
        let mut expr = self.parse_logical_and_expr(pairs[0].clone())?;
        
        for i in (1..pairs.len()).step_by(2) {
            // i est l'indice de l'opérateur || (implicite)
            let right = self.parse_logical_and_expr(pairs[i+1].clone())?;
            
            // Créer une nouvelle expression binaire
            let span = Span::new(
                expr.span().start,
                right.span().end,
            ).into();
            
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
                span,
            });
        }
        
        Ok(expr)
    }

    fn parse_logical_and_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        let pairs: Vec<_> = pair.into_inner().collect();
        
        if pairs.len() == 1 {
            // Pas d'opérateur AND, déléguer
            return self.parse_comparison_expr(pairs[0].clone());
        }
        
        // Construire une expression binaire pour chaque opérateur AND
        let mut expr = self.parse_comparison_expr(pairs[0].clone())?;
        
        for i in (1..pairs.len()).step_by(2) {
            // i est l'indice de l'opérateur && (implicite)
            let right = self.parse_comparison_expr(pairs[i+1].clone())?;
            
            // Créer une nouvelle expression binaire
            let span = Span::new(
                expr.span().start,
                right.span().end,
            ).into();
            
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
                span,
            });
        }
        
        Ok(expr)
    }

    fn parse_comparison_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        let mut inner_rules = pair.into_inner();
        
        // Première expression (gauche)
        let left_pair = inner_rules.next().unwrap();
        let left = self.parse_additive_expr(left_pair)?;
        
        // S'il n'y a pas d'opérateur de comparaison, retourner l'expression gauche
        let op_pair = match inner_rules.next() {
            Some(pair) => pair,
            None => return Ok(left),
        };
        
        // Convertir l'opérateur
        let op = match op_pair.as_str() {
            "==" => BinaryOp::Eq,
            "!=" => BinaryOp::NotEq,
            "<" => BinaryOp::Lt,
            "<=" => BinaryOp::LtEq,
            ">" => BinaryOp::Gt,
            ">=" => BinaryOp::GtEq,
            _ => return Err(CompilerError::ParserError {
                src: self.source.to_string(),
                span: self.get_span(&op_pair).into(),
                message: format!("Opérateur de comparaison invalide: {}", op_pair.as_str()),
            }),
        };
        
        // Expression droite
        let right_pair = inner_rules.next().unwrap();
        let right = self.parse_additive_expr(right_pair)?;
        
        // Créer l'expression binaire
        let span = Span::new(
            left.span().start,
            right.span().end,
        ).into();
        
        Ok(Expr::Binary(BinaryExpr {
            left: Box::new(left),
            op,
            right: Box::new(right),
            span,
        }))
    }

    fn parse_additive_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        let pairs: Vec<_> = pair.into_inner().collect();
        
        if pairs.len() == 1 {
            // Pas d'opérateur +/-, déléguer
            return self.parse_multiplicative_expr(pairs[0].clone());
        }
        
        // Construire une expression binaire pour chaque opérateur
        let mut expr = self.parse_multiplicative_expr(pairs[0].clone())?;
        
        for i in (1..pairs.len()).step_by(2) {
            // i est l'indice de l'opérateur +/-
            let op_str = pairs[i].as_str();
            let op = match op_str {
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Sub,
                _ => return Err(CompilerError::ParserError {
                    src: self.source.to_string(),
                    span: self.get_span(&pairs[i]).into(),
                    message: format!("Opérateur additif invalide: {}", op_str),
                }),
            };
            
            let right = self.parse_multiplicative_expr(pairs[i+1].clone())?;
            
            // Créer une nouvelle expression binaire
            let span = Span::new(
                expr.span().start,
                right.span().end,
            ).into();
            
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }
        
        Ok(expr)
    }

    fn parse_multiplicative_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        let pairs: Vec<_> = pair.into_inner().collect();
        
        if pairs.len() == 1 {
            // Pas d'opérateur */%, déléguer
            return self.parse_unary_expr(pairs[0].clone());
        }
        
        // Construire une expression binaire pour chaque opérateur
        let mut expr = self.parse_unary_expr(pairs[0].clone())?;
        
        for i in (1..pairs.len()).step_by(2) {
            // i est l'indice de l'opérateur */
            let op_str = pairs[i].as_str();
            let op = match op_str {
                "*" => BinaryOp::Mul,
                "/" => BinaryOp::Div,
                "%" => BinaryOp::Mod,
                _ => return Err(CompilerError::ParserError {
                    src: self.source.to_string(),
                    span: self.get_span(&pairs[i]).into(),
                    message: format!("Opérateur multiplicatif invalide: {}", op_str),
                }),
            };
            
            let right = self.parse_unary_expr(pairs[i+1].clone())?;
            
            // Créer une nouvelle expression binaire
            let span = Span::new(
                expr.span().start,
                right.span().end,
            ).into();
            
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }
        
        Ok(expr)
    }

    fn parse_unary_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        let mut inner_rules = pair.clone().into_inner();
        let first = inner_rules.next().unwrap();
        
        // Si c'est un opérateur unaire
        if let "+" | "-" | "!" = first.as_str() {
            let op = match first.as_str() {
                "+" => UnaryOp::Pos,
                "-" => UnaryOp::Neg,
                "!" => UnaryOp::Not,
                _ => unreachable!(),
            };
            
            let expr_pair = inner_rules.next().unwrap();
            let expr = Box::new(self.parse_unary_expr(expr_pair)?);
            
            let span = Span::new(
                self.get_source_pos(&pair),
                expr.span().end,
            ).into();
            
            Ok(Expr::Unary(UnaryExpr { op, expr, span }))
        } else {
            // Sinon c'est une expression primaire
            self.parse_primary_expr(pair)
        }
    }

    fn parse_primary_expr(&self, pair: Pair<Rule>) -> Result<Expr> {
        // Déléguer au premier enfant si nous sommes sur primary_expr
        let inner = if pair.as_rule() == Rule::primary_expr {
            pair.into_inner().next().unwrap()
        } else {
            pair
        };
        
        match inner.as_rule() {
            Rule::int_lit => {
                let span = self.get_span(&inner);
                let text = inner.as_str();
                let value = text.parse::<i32>().map_err(|_| {
                    CompilerError::ParserError {
                        src: self.source.to_string(),
                        span: span.into(),
                        message: format!("Entier invalide: {}", text),
                    }
                })?;
                
                Ok(Expr::Literal(Literal::Int(value, span.into())))
            },
            Rule::float_lit => {
                let span = self.get_span(&inner);
                let text = inner.as_str();
                let value = text.parse::<f64>().map_err(|_| {
                    CompilerError::ParserError {
                        src: self.source.to_string(),
                        span: span.into(),
                        message: format!("Flottant invalide: {}", text),
                    }
                })?;
                
                Ok(Expr::Literal(Literal::Float(value, span.into())))
            },
            Rule::string_lit => {
                let span = self.get_span(&inner);
                let text = inner.as_str();
                
                // Enlever les guillemets
                let value = text[1..text.len()-1].to_string();
                
                Ok(Expr::Literal(Literal::String(value, span.into())))
            },
            Rule::bool_lit => {
                let span = self.get_span(&inner);
                let text = inner.as_str();
                let value = text == "true";
                
                Ok(Expr::Literal(Literal::Bool(value, span.into())))
            },
            Rule::ident => {
                let span = self.get_span(&inner);
                let name = inner.as_str().to_string();
                
                Ok(Expr::Identifier(Identifier { name, span: span.into() }))
            },
            Rule::function_call => self.parse_function_call(inner),
            Rule::expr => self.parse_expr(inner),
            _ => Err(CompilerError::ParserError {
                src: self.source.to_string(),
                span: self.get_span(&inner).into(),
                message: format!("Expression primaire inattendue: {:?}", inner.as_rule()),
            }),
        }
    }

    fn parse_function_call(&self, pair: Pair<Rule>) -> Result<Expr> {
        let span = self.get_span(&pair);
        let mut inner_rules = pair.into_inner();
        
        let callee_pair = inner_rules.next().unwrap();
        let callee = callee_pair.as_str().to_string();
        
        let mut arguments = Vec::new();
        
        // Parcourir les arguments
        for arg_pair in inner_rules {
            arguments.push(self.parse_expr(arg_pair)?);
        }
        
        Ok(Expr::Call(FunctionCall {
            callee,
            arguments,
            span: span.into(),
        }))
    }

    // Fonction utilitaire pour convertir Pest Span en SourceSpan
    fn get_span(&self, pair: &Pair<Rule>) -> Span {
        let span = pair.as_span();
        Span::new(span.start(), span.end())
    }

    // Récupérer la position de début d'une paire
    fn get_source_pos(&self, pair: &Pair<Rule>) -> usize {
        pair.as_span().start()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let source = r#"
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }
        "#;
        
        let parser = AstParser::new(source);
        let result = parser.parse();
        
        assert!(result.is_ok());
        let program = result.unwrap();
        
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert_eq!(f.name, "add");
                assert_eq!(f.params.len(), 2);
                assert_eq!(f.params[0].name, "x");
                assert_eq!(f.params[1].name, "y");
            },
            _ => panic!("Expected function declaration"),
        }
    }

    #[test]
    fn test_parse_variable_declaration() {
        let source = "let x: i32 = 42;";
        
        let parser = AstParser::new(source);
        let result = parser.parse();
        
        assert!(result.is_ok());
        let program = result.unwrap();
        
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Variable(v) => {
                assert_eq!(v.name, "x");
                assert!(!v.is_mutable);
                match &v.typ {
                    Some(Type::I32) => {},
                    _ => panic!("Expected i32 type"),
                }
                match &v.initializer {
                    Expr::Literal(Literal::Int(42, _)) => {},
                    _ => panic!("Expected integer literal 42"),
                }
            },
            _ => panic!("Expected variable declaration"),
        }
    }
}