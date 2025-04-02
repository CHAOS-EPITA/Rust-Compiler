use logos::Logos;
use crate::common::{CompilerError, Result};
use super::tokens::{Token, TokenWithSpan};

/// Lexer analyse le code source et génère un flux de tokens
pub struct Lexer<'a> {
    source: &'a str,
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Crée un nouveau lexer à partir du code source
    pub fn new(source: &'a str) -> Self {
        let lexer = Token::lexer(source);
        Self { source, lexer }
    }

    /// Tokenise tout le code source et retourne un vecteur de tokens
    pub fn tokenize(&mut self) -> Result<Vec<TokenWithSpan>> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token() {
            match token {
                Ok(t) => tokens.push(t),
                Err(e) => return Err(e),
            }
        }
        
        Ok(tokens)
    }

    /// Retourne le prochain token ou None s'il n'y en a plus
    pub fn next_token(&mut self) -> Option<Result<TokenWithSpan>> {
        let token_result = self.lexer.next()?;
        let span = self.lexer.span();
        let text = self.lexer.slice().to_string();
        
        match token_result {
            Ok(Token::Error) => {
                Some(Err(CompilerError::LexerError {
                    src: self.source.to_string(),
                    span: (span.start, span.end - span.start).into(),
                    message: format!("Caractère invalide: '{}'", text),
                }))
            },
            Ok(token) => Some(Ok(TokenWithSpan::new(token, (span.start, span.end), text))),
            Err(_) => Some(Err(CompilerError::LexerError {
                src: self.source.to_string(), 
                span: (span.start, span.end - span.start).into(),
                message: format!("Erreur lors de l'analyse lexicale: '{}'", text),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_simple() {
        let source = "let x = 5;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::Let);
        assert_eq!(tokens[1].token, Token::Ident);
        assert_eq!(tokens[2].token, Token::Assign);
        assert_eq!(tokens[3].token, Token::IntLiteral);
        assert_eq!(tokens[4].token, Token::Semicolon);
    }
    
    #[test]
    fn test_lexer_function() {
        let source = "fn add(x: i32, y: i32) -> i32 { return x + y; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(tokens.len() > 10);
        assert_eq!(tokens[0].token, Token::Fn);
        assert_eq!(tokens[1].token, Token::Ident);
        assert_eq!(tokens[2].token, Token::LParen);
    }
}