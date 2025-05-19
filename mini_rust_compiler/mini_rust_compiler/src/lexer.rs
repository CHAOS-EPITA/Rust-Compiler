// filepath: /mini_rust_compiler/mini_rust_compiler/src/lexer.rs
use crate::error_handler::ErrorHandler;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Fn,
    Let,
    Mut,
    Return,
    If,
    Else,
    While,
    
    // Types
    I32,
    
    // Identifiers and literals
    Identifier(String),
    IntLiteral(i32),
    StringLiteral(String),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Assign,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Arrow,
    
    // Macro specific
    PrintlnMacro,
    
    // End of file
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

#[derive(Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    position: usize,
    line: usize,
    error_handler: &'a ErrorHandler,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, error_handler: &'a ErrorHandler) -> Self {
        Lexer {
            source,
            chars: source.chars().collect(),
            position: 0,
            line: 1,
            error_handler,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, usize> {
        let mut tokens = Vec::new();
        
        while self.position < self.chars.len() {
            match self.next_token() {
                Ok(token) => tokens.push(token),
                Err(line) => return Err(line),
            }
        }
        
        // Add EOF token
        tokens.push(Token {
            token_type: TokenType::EOF,
            line: self.line,
        });
        
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token, usize> {
        self.skip_whitespace();
        
        if self.position >= self.chars.len() {
            return Ok(Token {
                token_type: TokenType::EOF,
                line: self.line,
            });
        }
        
        let c = self.chars[self.position];
        self.position += 1;
        
        match c {
            // Simple operators
            '+' => Ok(Token { token_type: TokenType::Plus, line: self.line }),
            '-' => {
                if self.match_char('>') {
                    Ok(Token { token_type: TokenType::Arrow, line: self.line })
                } else {
                    Ok(Token { token_type: TokenType::Minus, line: self.line })
                }
            },
            '*' => Ok(Token { token_type: TokenType::Star, line: self.line }),
            '/' => Ok(Token { token_type: TokenType::Slash, line: self.line }),
            '%' => Ok(Token { token_type: TokenType::Mod, line: self.line }),
            
            // Punctuation
            '(' => Ok(Token { token_type: TokenType::LeftParen, line: self.line }),
            ')' => Ok(Token { token_type: TokenType::RightParen, line: self.line }),
            '{' => Ok(Token { token_type: TokenType::LeftBrace, line: self.line }),
            '}' => Ok(Token { token_type: TokenType::RightBrace, line: self.line }),
            ',' => Ok(Token { token_type: TokenType::Comma, line: self.line }),
            ';' => Ok(Token { token_type: TokenType::Semicolon, line: self.line }),
            ':' => Ok(Token { token_type: TokenType::Colon, line: self.line }),
            
            // Compound operators
            '=' => {
                if self.match_char('=') {
                    Ok(Token { token_type: TokenType::Equal, line: self.line })
                } else {
                    Ok(Token { token_type: TokenType::Assign, line: self.line })
                }
            },
            '!' => {
                if self.match_char('=') {
                    Ok(Token { token_type: TokenType::NotEqual, line: self.line })
                } else {
                    Err(self.line)
                }
            },
            '<' => {
                if self.match_char('=') {
                    Ok(Token { token_type: TokenType::LessEqual, line: self.line })
                } else {
                    Ok(Token { token_type: TokenType::Less, line: self.line })
                }
            },
            '>' => {
                if self.match_char('=') {
                    Ok(Token { token_type: TokenType::GreaterEqual, line: self.line })
                } else {
                    Ok(Token { token_type: TokenType::Greater, line: self.line })
                }
            },
            
            // String literals
            '"' => self.string(),
            
            // Numbers or identifiers
            '0'..='9' => {
                self.position -= 1;
                self.number()
            },
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                self.position -= 1;
                self.identifier()
            },
            
            // Other characters
            _ => Err(self.line),
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.position < self.chars.len() {
            match self.chars[self.position] {
                ' ' | '\r' | '\t' => {
                    self.position += 1;
                },
                '\n' => {
                    self.position += 1;
                    self.line += 1;
                },
                // Comments
                '/' => {
                    if self.position + 1 < self.chars.len() && self.chars[self.position + 1] == '/' {
                        while self.position < self.chars.len() && self.chars[self.position] != '\n' {
                            self.position += 1;
                        }
                    } else {
                        return;
                    }
                },
                _ => return,
            }
        }
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.position >= self.chars.len() || self.chars[self.position] != expected {
            return false;
        }
        
        self.position += 1;
        true
    }
    
    fn string(&mut self) -> Result<Token, usize> {
        let start = self.position;
        
        while self.position < self.chars.len() && self.chars[self.position] != '"' {
            if self.chars[self.position] == '\n' {
                self.line += 1;
            }
            self.position += 1;
        }
        
        if self.position >= self.chars.len() {
            return Err(self.line);
        }
        
        // Consume the closing quote
        self.position += 1;
        
        let value = self.chars[start..self.position - 1].iter().collect();
        
        Ok(Token {
            token_type: TokenType::StringLiteral(value),
            line: self.line,
        })
    }
    
    fn number(&mut self) -> Result<Token, usize> {
        let start = self.position;
        
        while self.position < self.chars.len() && self.chars[self.position].is_digit(10) {
            self.position += 1;
        }
        
        let number_str: String = self.chars[start..self.position].iter().collect();
        let value = number_str.parse::<i32>().map_err(|_| self.line)?;
        
        Ok(Token {
            token_type: TokenType::IntLiteral(value),
            line: self.line,
        })
    }
    
    fn identifier(&mut self) -> Result<Token, usize> {
        let start = self.position;
        
        while self.position < self.chars.len() && 
              (self.chars[self.position].is_alphanumeric() || self.chars[self.position] == '_') {
            self.position += 1;
        }
        
        let text: String = self.chars[start..self.position].iter().collect();
        
        // Check if it's a keyword
        let token_type = match text.as_str() {
            "fn" => TokenType::Fn,
            "let" => TokenType::Let,
            "mut" => TokenType::Mut,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "i32" => TokenType::I32,
            "println" => {
                // Handle macros like println!
                if self.position < self.chars.len() && self.chars[self.position] == '!' {
                    self.position += 1;
                    TokenType::PrintlnMacro
                } else {
                    TokenType::Identifier(text)
                }
            },
            _ => TokenType::Identifier(text),
        };
        
        Ok(Token {
            token_type,
            line: self.line,
        })
    }
}