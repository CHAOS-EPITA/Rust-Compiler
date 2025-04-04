mod lexer;
mod parser;
mod code_generator;
mod error_handler;

use std::env;
use std::fs;
use std::process;

fn main() {
    // Récupérer les arguments de ligne de commande
    let args: Vec<String> = env::args().collect();
    
    // Vérifier que le fichier source est fourni
    if args.len() < 2 {
        eprintln!("Usage: {} <fichier.rs>", args[0]);
        process::exit(1);
    }
    
    // Lire le fichier source
    let source_path = &args[1];
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erreur lors de la lecture du fichier {}: {}", source_path, e);
            process::exit(1);
        }
    };
    
    // Créer l'error handler
    let error_handler = error_handler::ErrorHandler::new(source_path.clone());
    
    // Lexer: transformer le code source en tokens
    let mut lexer = lexer::Lexer::new(&source_code, &error_handler);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(line) => {
            error_handler.report_error(line, "Erreur lexicale");
            process::exit(1);
        }
    };
    
    // Parser: créer l'arbre syntaxique abstrait
    let mut parser = parser::Parser::new(tokens, &error_handler);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(line) => {
            error_handler.report_error(line, "Erreur syntaxique");
            process::exit(1);
        }
    };
    
    // Générateur de code: produire du code machine à partir de l'AST
    let mut code_gen = code_generator::CodeGenerator::new(&error_handler);
    let executable_path = match code_gen.generate(ast, source_path) {
        Ok(path) => path,
        Err(line) => {
            error_handler.report_error(line, "Erreur de génération de code");
            process::exit(1);
        }
    };
    
    //println!("Compilation réussie ! Exécutable généré: {}", executable_path);
}