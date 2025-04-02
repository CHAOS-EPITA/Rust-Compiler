use std::fs;
use std::path::Path;
use std::process::Command;
use clap::Parser;
use miette::{Result, IntoDiagnostic};

mod lexer;
mod parser;
mod typer;
mod codegen;
mod common;

use lexer::lexer::Lexer;
use parser::parser::AstParser;
use typer::TypeChecker;
use codegen::codegen::CodeGenerator;

/// Compilateur Rust simplifié
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Fichier source rust (.rs)
    #[arg(short, long)]
    input: String,

    /// Fichier assembleur de sortie (.s)
    #[arg(short, long, default_value_t = String::from("output.s"))]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Compilation de {} vers {}", args.input, args.output);
    
    // Lire le code source
    let source = fs::read_to_string(&args.input).into_diagnostic()?;
    
    // Phase 1: Analyse lexicale
    println!("1. Analyse lexicale...");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    
    println!("  Nombre de tokens: {}", tokens.len());
    
    // Phase 2: Analyse syntaxique
    println!("2. Analyse syntaxique...");
    let parser = AstParser::new(&source);
    let ast = parser.parse()?;
    
    println!("  Nombre de déclarations: {}", ast.declarations.len());
    
    // Phase 3: Vérification des types
    println!("3. Vérification des types...");
    let mut type_checker = TypeChecker::new(source.clone());
    type_checker.check_program(&ast)?;
    
    // Phase 4: Génération de code
    println!("4. Génération de code assembleur...");
    let code_gen = CodeGenerator::new();
    let asm_code = code_gen.generate(&ast)?;
    
    // Écrire dans le fichier de sortie
    fs::write(&args.output, asm_code).into_diagnostic()?;
    
    // Assembler et lier le code si le fichier de sortie a l'extension .s
    if args.output.ends_with(".s") {
        println!("5. Assemblage et liaison...");
        assemble_and_link(&args.output)?;
    }
    
    println!("Compilation terminée avec succès!");
    Ok(())
}

/// Assemble et lie le code assembleur généré
fn assemble_and_link(asm_file: &str) -> Result<()> {
    let output_path = Path::new(asm_file);
    let output_name = output_path.file_stem().unwrap().to_str().unwrap();
    
    // Assembler
    let object_file = format!("{}.o", output_name);
    let status = Command::new("nasm")
        .args(["-f", "elf64", asm_file, "-o", &object_file])
        .status()
        .into_diagnostic()?;
    
    if !status.success() {
        return Err(miette::miette!("Erreur lors de l'assemblage"));
    }
    
    // Lier
    let executable = output_name;
    let status = Command::new("ld")
        .args([&object_file, "-o", executable])
        .status()
        .into_diagnostic()?;
    
    if !status.success() {
        return Err(miette::miette!("Erreur lors de la liaison"));
    }
    
    println!("Exécutable créé: ./{}", executable);
    println!("Vous pouvez l'exécuter avec: ./{}", executable);
    
    Ok(())
}
