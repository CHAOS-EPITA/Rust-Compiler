use crate::error_handler::ErrorHandler;
use crate::lexer::{Token, TokenType};

// Définition des structures pour l'AST (Abstract Syntax Tree)
#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
    Variable(String),
    Call(String, Vec<Expr>),
    FunctionCall(String, Vec<Expr>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    String,
    Void,
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Let(String, Option<Expr>, bool, Type), // Nom, valeur initiale, mutable, type
    Assign(String, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(String, Expr, Expr, Box<Stmt>),  // Variable, range_start, range_end, body
    Return(Option<Expr>),
    Println(Vec<Expr>),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>, // (nom, type)
    pub return_type: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    error_handler: &'a ErrorHandler,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_handler: &'a ErrorHandler) -> Self {
        Parser {
            tokens,
            current: 0,
            error_handler,
        }
    }
    
    pub fn parse(&mut self) -> Result<Program, usize> {
        let mut program = Program { functions: Vec::new() };
        
        while !self.is_at_end() {
            match self.function() {
                Ok(function) => program.functions.push(function),
                Err(line) => return Err(line),
            }
        }
        
        Ok(program)
    }
    
    fn function(&mut self) -> Result<Function, usize> {
        self.consume(TokenType::Fn, "Attendu 'fn'")?;
        
        let name = match &self.peek().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err(self.peek().line),
        };
        self.advance();
        
        self.consume(TokenType::LeftParen, "Attendu '(' après le nom de la fonction")?;
        
        // Paramètres
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                let param_name = match &self.peek().token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err(self.peek().line),
                };
                self.advance();
                
                self.consume(TokenType::Colon, "Attendu ':' après le nom du paramètre")?;
                
                let param_type = match &self.peek().token_type {
                    TokenType::I32 => "i32".to_string(),
                    TokenType::Identifier(type_name) => type_name.clone(),
                    _ => return Err(self.peek().line),
                };
                self.advance();
                
                params.push((param_name, param_type));
                
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(TokenType::RightParen, "Attendu ')' après les paramètres")?;
        
        // Type de retour
        let return_type = if self.match_token(TokenType::Arrow) {
            if let TokenType::I32 = self.peek().token_type {
                self.advance();
                Some("i32".to_string())
            } else if let TokenType::Identifier(ref type_name) = self.peek().token_type {
                let type_name = type_name.clone();
                self.advance();
                Some(type_name)
            } else {
                return Err(self.peek().line);
            }
        } else {
            None
        };
        
        // Corps de la fonction
        self.consume(TokenType::LeftBrace, "Attendu '{' avant le corps de la fonction")?;
        
        let mut body = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => body.push(stmt),
                Err(line) => return Err(line),
            }
        }
        
        self.consume(TokenType::RightBrace, "Attendu '}' après le corps de la fonction")?;
        
        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }
    
    fn statement(&mut self) -> Result<Stmt, usize> {
        if self.match_token(TokenType::Let) {
            self.let_statement()
        } else if self.match_token(TokenType::Return) {
            self.return_statement()
        } else if self.match_token(TokenType::LeftBrace) {
            self.block_statement()
        } else if self.match_token(TokenType::If) {
            self.if_statement()
        } else if self.match_token(TokenType::While) {
            self.while_statement()
        } else if self.match_token(TokenType::For) {
            self.for_statement()  // Add for statement handling
        } else if self.check(TokenType::PrintlnMacro) {
            self.println_statement()
        } else {
            self.expression_statement()
        }
    }
    
    fn let_statement(&mut self) -> Result<Stmt, usize> {
        let mutable = self.match_token(TokenType::Mut);
        
        let name = match &self.peek().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err(self.peek().line),
        };
        self.advance();
        
        // Parse le type
        let var_type = if self.match_token(TokenType::Colon) {
            self.type_annotation()?
        } else {
            Type::I32  // Type par défaut
        };
        
        // Initialisation
        let initializer = if self.match_token(TokenType::Assign) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(TokenType::Semicolon, "Attendu ';' après la déclaration")?;
        
        Ok(Stmt::Let(name, initializer, mutable, var_type))  // Include type
    }
    
    fn return_statement(&mut self) -> Result<Stmt, usize> {
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(TokenType::Semicolon, "Attendu ';' après l'instruction return")?;
        
        Ok(Stmt::Return(value))
    }
    
    fn block_statement(&mut self) -> Result<Stmt, usize> {
        let mut statements = Vec::new();
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }
        
        self.consume(TokenType::RightBrace, "Attendu '}' après le bloc")?;
        
        Ok(Stmt::Block(statements))
    }
    
    fn if_statement(&mut self) -> Result<Stmt, usize> {
        // Parse condition without requiring parentheses
        let condition = self.expression()?;
        
        self.consume(TokenType::LeftBrace, "Attendu '{' après la condition if")?;
        let then_branch = Box::new(self.block_statement()?);
        
        let else_branch = if self.match_token(TokenType::Else) {
            if self.match_token(TokenType::If) {
                // Handle "else if" as nested if statement
                Some(Box::new(self.if_statement()?))
            } else {
                // Regular else block
                self.consume(TokenType::LeftBrace, "Attendu '{' après 'else'")?;
                Some(Box::new(self.block_statement()?))
            }
        } else {
            None
        };
        
        Ok(Stmt::If(condition, then_branch, else_branch))
    }
    
    fn while_statement(&mut self) -> Result<Stmt, usize> {
        // Also update while to not require parentheses
        let condition = self.expression()?;
        
        self.consume(TokenType::LeftBrace, "Attendu '{' après la condition while")?;
        let body = Box::new(self.block_statement()?);
        
        Ok(Stmt::While(condition, body))
    }
    
    // New method for parsing for loops
    fn for_statement(&mut self) -> Result<Stmt, usize> {
        // Get the loop variable name
        let var_name = match &self.peek().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => {
                self.error_handler.report_error(self.peek().line, "Expected identifier after 'for'");
                return Err(self.peek().line);
            }
        };
        self.advance();
        
        // Expect 'in' keyword
        self.consume(TokenType::In, "Expected 'in' after identifier in for loop")?;
        
        // Parse range start expression
        let range_start = self.expression()?;
        
        // Expect '..' token
        self.consume(TokenType::DotDot, "Expected '..' in range expression")?;
        
        // Parse range end expression
        let range_end = self.expression()?;
        
        // Parse loop body
        self.consume(TokenType::LeftBrace, "Expected '{' before for loop body")?;
        let body = self.block_statement()?;
        
        // Wrap the body in a Box
        if let Stmt::Block(statements) = body {
            Ok(Stmt::For(var_name, range_start, range_end, Box::new(Stmt::Block(statements))))
        } else {
            // This shouldn't happen since block_statement always returns a Stmt::Block
            Err(self.peek().line)
        }
    }
    
    fn println_statement(&mut self) -> Result<Stmt, usize> {
        self.advance(); // Consommer println!
        
        self.consume(TokenType::LeftParen, "Attendu '(' après 'println!'")?;
        
        let mut args = Vec::new();
        if let TokenType::StringLiteral(_) = &self.peek().token_type {
            args.push(self.expression()?);
            
            while self.match_token(TokenType::Comma) {
                args.push(self.expression()?);
            }
        }
        
        self.consume(TokenType::RightParen, "Attendu ')' après les arguments")?;
        self.consume(TokenType::Semicolon, "Attendu ';' après l'appel à println!")?;
        
        Ok(Stmt::Println(args))
    }
    
    fn expression_statement(&mut self) -> Result<Stmt, usize> {
        let expr = self.expression()?;
        
        self.consume(TokenType::Semicolon, "Attendu ';' après l'expression")?;
        
        Ok(Stmt::Expression(expr))
    }
    
    fn expression(&mut self) -> Result<Expr, usize> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expr, usize> {
        let expr = self.equality()?;
        
        if self.match_token(TokenType::Assign) {
            let value = self.assignment()?;
            
            if let Expr::Variable(name) = expr {
                return Ok(Expr::Binary(
                    Box::new(Expr::Variable(name)),
                    BinaryOp::Equal,
                    Box::new(value),
                ));
            }
            
            return Err(self.previous().line);
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, usize> {
        let mut expr = self.comparison()?;
        
        while self.match_any(&[TokenType::Equal, TokenType::NotEqual]) {
            let operator = match &self.previous().token_type {
                TokenType::Equal => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, usize> {
        let mut expr = self.term()?;
        
        while self.match_any(&[
            TokenType::Less, TokenType::LessEqual,
            TokenType::Greater, TokenType::GreaterEqual,
        ]) {
            let operator = match &self.previous().token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, usize> {
        let mut expr = self.factor()?;
        
        while self.match_any(&[TokenType::Plus, TokenType::Minus]) {
            let operator = match &self.previous().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, usize> {
        let mut expr = self.unary()?;
        
        while self.match_any(&[TokenType::Star, TokenType::Slash, TokenType::Mod]) {
            let operator = match &self.previous().token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Mod => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, usize> {
        if self.match_token(TokenType::Minus) {
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryOp::Negate, Box::new(right)));
        }
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expr, usize> {
        let mut expr = self.primary()?;
        
        if self.check(TokenType::LeftParen) {
            if let Expr::Variable(callee) = expr {
                self.advance(); // Consommer (
                
                let mut arguments = Vec::new();
                if !self.check(TokenType::RightParen) {
                    loop {
                        arguments.push(self.expression()?);
                        
                        if !self.match_token(TokenType::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(TokenType::RightParen, "Attendu ')' après les arguments")?;
                
                expr = Expr::FunctionCall(callee, arguments);
            } else {
                return Err(self.peek().line);
            }
        }
        
        Ok(expr)
    }
    
    fn primary(&mut self) -> Result<Expr, usize> {
        if let TokenType::IntLiteral(value) = &self.peek().token_type {
            let value = *value;
            self.advance();
            return Ok(Expr::Literal(Literal::Int(value)));
        } else if let TokenType::StringLiteral(value) = &self.peek().token_type.clone() {
            let value = value.clone();
            self.advance();
            return Ok(Expr::Literal(Literal::String(value)));
        } else if let TokenType::Identifier(name) = &self.peek().token_type.clone() {
            let name = name.clone();
            self.advance();
            return Ok(Expr::Variable(name));
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Attendu ')' après l'expression")?;
            return Ok(expr);
        }
        
        Err(self.peek().line)
    }
    
    // Méthodes utilitaires
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }
    
    fn match_any(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        match (&self.peek().token_type, &token_type) {
            (TokenType::IntLiteral(_), TokenType::IntLiteral(_)) => true,
            (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (a, b) => a == b,
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, usize> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            self.error_handler.report_error(self.peek().line, message);
            Err(self.peek().line)
        }
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EOF)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn type_annotation(&mut self) -> Result<Type, usize> {
        match &self.peek().token_type {
            TokenType::I32 => {
                self.advance();
                Ok(Type::I32)
            },
            TokenType::Identifier(type_name) => {
                match type_name.as_str() {
                    "i8" => {
                        self.advance();
                        Ok(Type::I8)
                    },
                    "i16" => {
                        self.advance();
                        Ok(Type::I16)
                    },
                    "i32" => {
                        self.advance();
                        Ok(Type::I32)
                    },
                    "i64" => {
                        self.advance();
                        Ok(Type::I64)
                    },
                    "i128" => {
                        self.advance();
                        Ok(Type::I128)
                    },
                    "f32" => {
                        self.advance();
                        Ok(Type::F32)
                    },
                    "f64" => {
                        self.advance();
                        Ok(Type::F64)
                    },
                    "String" => {
                        self.advance();
                        Ok(Type::String)
                    },
                    _ => {
                        self.error_handler.report_error(self.peek().line, &format!("Type inconnu: {}", type_name));
                        Err(self.peek().line)
                    }
                }
            },
            _ => {
                self.error_handler.report_error(self.peek().line, "Type attendu");
                Err(self.peek().line)
            }
        }
    }
}