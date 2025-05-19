// This file contains unit tests for the compiler's components, such as the lexer, parser, and code generator, ensuring that each part functions as expected.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Lexer, TokenType};
    use crate::parser::Parser;
    use crate::code_generator::CodeGenerator;
    use crate::error_handler::ErrorHandler;

    #[test]
    fn test_lexer_tokenization() {
        let source = "let x: i32 = 10;";
        let error_handler = ErrorHandler::new("test.rs".to_string());
        let mut lexer = Lexer::new(source, &error_handler);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("x".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::I32);
        assert_eq!(tokens[4].token_type, TokenType::Assign);
        assert_eq!(tokens[5].token_type, TokenType::IntLiteral(10));
    }

    #[test]
    fn test_parser_function_parsing() {
        let source = "fn sum(x: i32, y: i32) { return x + y; }";
        let error_handler = ErrorHandler::new("test.rs".to_string());
        let mut lexer = Lexer::new(source, &error_handler);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens, &error_handler);
        let program = parser.parse().unwrap();

        assert_eq!(program.functions.len(), 1);
        let function = &program.functions[0];
        assert_eq!(function.name, "sum");
        assert_eq!(function.params.len(), 2);
        assert_eq!(function.return_type, Some("i32".to_string()));
    }

    #[test]
    fn test_code_generator() {
        let source = "fn main() { let x: i32 = 10; }";
        let error_handler = ErrorHandler::new("test.rs".to_string());
        let mut lexer = Lexer::new(source, &error_handler);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens, &error_handler);
        let program = parser.parse().unwrap();
        let mut code_gen = CodeGenerator::new(&error_handler);
        let result = code_gen.generate(program, "output.c");

        assert!(result.is_ok());
    }
}