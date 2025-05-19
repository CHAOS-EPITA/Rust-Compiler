// filepath: /home/sany/Rust-Compiler/mini_rust_compiler/src/parser.rs
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

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Let(String, Option<Expr>, bool), // Nom, valeur initiale, mutable
    Assign(String, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
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
            program.functions.push(self.function()?);
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
                    TokenType::I32 => {
                        self.advance();
                        "i32".to_string()
                    },
                    _ => return Err(self.peek().line),
                };
                
                params.push((param_name, param_type));
                
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(TokenType::RightParen, "Attendu ')' après les paramètres")?;
        
        // Type de retour
        let return_type = if self.match_token(TokenType::Arrow) {
            match &self.peek().token_type {
                TokenType::I32 => {
                    self.advance();
                    Some("i32".to_string())
                },
                _ => return Err(self.peek().line),
            }
        } else {
            None
        };
        
        // Corps de la fonction
        self.consume(TokenType::LeftBrace, "Attendu '{' avant le corps de la fonction")?;
        
        let mut body = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
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
        
        // Type
        self.consume(TokenType::Colon, "Attendu ':' après le nom de la variable")?;
        if let TokenType::I32 = self.peek().token_type {
            self.advance();
        } else {
            return Err(self.peek().line);
        }
        
        // Initialisation
        let initializer = if self.match_token(TokenType::Assign) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(TokenType::Semicolon, "Attendu ';' après la déclaration")?;
        
        Ok(Stmt::Let(name, initializer, mutable))
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
        self.consume(TokenType::LeftParen, "Attendu '(' après 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Attendu ')' après la condition")?;
        
        let then_branch = Box::new(self.statement()?);
        
        let else_branch = if self.match_token(TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        
        Ok(Stmt::If(condition, then_branch, else_branch))
    }
    
    fn while_statement(&mut self) -> Result<Stmt, usize> {
        self.consume(TokenType::LeftParen, "Attendu '(' après 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Attendu ')' après la condition")?;
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::While(condition, body))
    }
    
    fn println_statement(&mut self) -> Result<Stmt, usize> {
        self.advance(); // Consommer println!
        
        self.consume(TokenType::LeftParen, "Attendu '(' après 'println!'")?;
        
        let mut args = Vec::new();
        if let TokenType::StringLiteral(_) = &self.peek().token_type {
            args.push(self.expression()?);
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
            return Ok(Expr::Binary(Box::new(expr), BinaryOp::Equal, Box::new(value)));
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, usize> {
        let mut expr = self.comparison()?;
        
        while self.match_any(&[TokenType::Equal, TokenType::NotEqual]) {
            let operator = if self.previous().token_type == TokenType::Equal {
                BinaryOp::Equal
            } else {
                BinaryOp::NotEqual
            };
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, usize> {
        let mut expr = self.term()?;
        
        while self.match_any(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = match self.previous().token_type {
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
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
            let operator = if self.previous().token_type == TokenType::Plus {
                BinaryOp::Add
            } else {
                BinaryOp::Subtract
            };
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, usize> {
        let mut expr = self.unary()?;
        
        while self.match_any(&[TokenType::Star, TokenType::Slash, TokenType::Mod]) {
            let operator = match self.previous().token_type {
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
        
        while self.check(TokenType::LeftParen) {
            self.consume(TokenType::LeftParen, "Attendu '(' après l'identifiant")?;
            let mut args = Vec::new();
            if !self.check(TokenType::RightParen) {
                loop {
                    args.push(self.expression()?);
                    if !self.match_token(TokenType::Comma) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Attendu ')' après les arguments")?;
            expr = Expr::FunctionCall(expr.to_string(), args);
        }
        
        Ok(expr)
    }
    
    fn primary(&mut self) -> Result<Expr, usize> {
        if let TokenType::IntLiteral(value) = &self.peek().token_type {
            self.advance();
            return Ok(Expr::Literal(Literal::Int(*value)));
        } else if let TokenType::StringLiteral(value) = &self.peek().token_type {
            self.advance();
            return Ok(Expr::Literal(Literal::String(value.clone())));
        } else if let TokenType::Identifier(name) = &self.peek().token_type {
            self.advance();
            return Ok(Expr::Variable(name.clone()));
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Attendu ')' après l'expression")?;
            return Ok(expr);
        }
        
        Err(self.peek().line)
    }
    
    // Méthodes utilitaires
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek().token_type != token_type {
            return false;
        }
        self.advance();
        true
    }
    
    fn match_any(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.match_token(token_type.clone()) {
                return true;
            }
        }
        false
    }
    
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, usize> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.peek().line)
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}