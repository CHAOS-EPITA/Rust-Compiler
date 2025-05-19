mod lexer;
mod parser;
mod code_generator;
mod error_handler;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file.rs>", args[0]);
        process::exit(1);
    }
    
    let source_path = &args[1];
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", source_path, e);
            process::exit(1);
        }
    };
    
    let error_handler = error_handler::ErrorHandler::new(source_path.clone());
    
    let mut lexer = lexer::Lexer::new(&source_code, &error_handler);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(line) => {
            error_handler.report_error(line, "Lexical error");
            process::exit(1);
        }
    };
    
    let mut parser = parser::Parser::new(tokens, &error_handler);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(line) => {
            error_handler.report_error(line, "Syntax error");
            process::exit(1);
        }
    };
    
    let mut code_gen = code_generator::CodeGenerator::new(&error_handler);
    let executable_path = match code_gen.generate(ast, source_path) {
        Ok(path) => path,
        Err(line) => {
            error_handler.report_error(line, "Code generation error");
            process::exit(1);
        }
    };
    
    println!("Compilation successful! Executable generated: {}", executable_path);
}