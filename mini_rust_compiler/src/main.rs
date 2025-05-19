mod lexer;
mod parser;
mod code_generator;
mod error_handler;

use std::env;
use std::fs;
use std::process::{self, Command};
use std::time::Instant;

fn main() {
    // Vérifier que les outils nécessaires sont installés
    check_required_tools();
    
    // Récupérer les arguments de ligne de commande
    let args: Vec<String> = env::args().collect();
    
    // Vérifier que le fichier source est fourni
    if args.len() < 2 {
        eprintln!("Usage: {} <fichier.rs>", args[0]);
        process::exit(1);
    }
    
    let start_time = Instant::now();
    
    // Lire le fichier source
    let source_path = &args[1];
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erreur lors de la lecture du fichier {}: {}", source_path, e);
            process::exit(1);
        }
    };
    
    println!("Compilation de {}...", source_path);
    
    // Créer l'error handler
    let error_handler = error_handler::ErrorHandler::new(source_path.clone());
    
    // Lexer: transformer le code source en tokens
    println!("Étape 1/3: Analyse lexicale...");
    let mut lexer = lexer::Lexer::new(&source_code, &error_handler);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(line) => {
            error_handler.report_error(line, "Erreur lexicale");
            process::exit(1);
        }
    };
    
    // Parser: créer l'arbre syntaxique abstrait
    println!("Étape 2/3: Analyse syntaxique...");
    let mut parser = parser::Parser::new(tokens, &error_handler);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(line) => {
            error_handler.report_error(line, "Erreur syntaxique");
            process::exit(1);
        }
    };
    
    // Générateur de code: produire du code machine à partir de l'AST
    println!("Étape 3/3: Génération de code et compilation...");
    let mut code_gen = code_generator::CodeGenerator::new(&error_handler);
    let executable_path = match code_gen.generate(ast, source_path) {
        Ok(path) => path,
        Err(line) => {
            error_handler.report_error(line, "Erreur de génération de code");
            process::exit(1);
        }
    };
    
    let duration = start_time.elapsed();
    println!("Compilation terminée avec succès en {:.2?}", duration);
    println!("Exécutable généré: {}", executable_path);
}

fn check_required_tools() {
    // Vérifier que NASM est installé
    if Command::new("nasm").arg("--version").output().is_err() {
        eprintln!("ERREUR: NASM (assembleur x86_64) n'est pas installé.");
        eprintln!("Veuillez l'installer avec: sudo apt-get install nasm");
        process::exit(1);
    }
    
    // Vérifier que LD est installé
    if Command::new("ld").arg("--version").output().is_err() {
        eprintln!("ERREUR: LD (éditeur de liens) n'est pas installé.");
        eprintln!("Veuillez installer les outils de compilation avec: sudo apt-get install binutils");
        process::exit(1);
    }
}