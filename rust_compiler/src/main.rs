mod parser;
use crate::parser::lexer;
use crate::parser::parser::Parser;
use crate::parser::ast::ASTNode;

use std::fs::File;
use std::io::{self, Write, BufRead};
use std::process::Command;

fn main() {
    println!("=== Parser & AST Visualizer ===");
    
    // Expressions prédéfinies pour démonstration
    let examples = vec![
        "2 + 3",
        "2 * 3",
        "2 + 3 * 4",
        "(2 + 3) * 4",
        "2 * (3 + 4)",
        "5 - 3 + 2",
        "10 / 2 - 3",
        "2 * (3 + 4 * 5) + 6"
    ];
    
    loop {
        println!("\nChoisissez une option :");
        println!("1. Visualiser une expression prédéfinie");
        println!("2. Entrer une expression personnalisée");
        println!("3. Quitter");
        
        print!("Votre choix : ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().lock().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => {
                println!("\nExemples disponibles :");
                for (i, expr) in examples.iter().enumerate() {
                    println!("{}. {}", i + 1, expr);
                }
                
                print!("Choisissez un exemple (1-{}) : ", examples.len());
                io::stdout().flush().unwrap();
                
                let mut example_choice = String::new();
                io::stdin().lock().read_line(&mut example_choice).unwrap();
                
                if let Ok(idx) = example_choice.trim().parse::<usize>() {
                    if idx >= 1 && idx <= examples.len() {
                        visualize_expression(examples[idx - 1]);
                    } else {
                        println!("Choix invalide.");
                    }
                } else {
                    println!("Veuillez entrer un nombre valide.");
                }
            },
            "2" => {
                print!("\nEntrez votre expression : ");
                io::stdout().flush().unwrap();
                
                let mut custom_expr = String::new();
                io::stdin().lock().read_line(&mut custom_expr).unwrap();
                
                visualize_expression(custom_expr.trim());
            },
            "3" => {
                println!("Au revoir !");
                break;
            },
            _ => println!("Option invalide. Veuillez réessayer."),
        }
    }
}

fn visualize_expression(input: &str) {
    println!("\nExpression : {}", input);
    
    // Tokenization
    let tokens = lexer::tokenize(input);
    println!("Tokens : {:?}", tokens);
    
    // Parsing
    let mut parser = Parser::new(tokens);
    
    match parser.parse() {
        Some(ast) => {
            println!("Parsing réussi !");
            
            // Affichage de l'AST
            print_ast(&ast, 0);
            
            // Évaluation
            println!("Résultat : {}", ast.evaluate());
            
            // Génération de la visualisation graphique
            let dot_content = ast.to_dot();
            let filename = sanitize_filename(input);
            
            let dot_file = format!("{}.dot", filename);
            let png_file = format!("{}.png", filename);
            
            // Écriture du fichier DOT
            let mut file = File::create(&dot_file).expect("Failed to create file");
            file.write_all(dot_content.as_bytes()).expect("Failed to write to file");
            
            // Génération de l'image
            match Command::new("dot")
                .args(["-Tpng", &dot_file, "-o", &png_file])
                .status() {
                Ok(_) => println!("Visualisation enregistrée dans {}", png_file),
                Err(_) => println!("Erreur lors de la génération de l'image. Vérifiez que Graphviz est installé."),
            }
        },
        None => println!("Erreur : impossible de parser l'expression"),
    }
}

// Fonction pour afficher l'AST de manière hiérarchique dans la console
fn print_ast(node: &ASTNode, level: usize) {
    let indent = "  ".repeat(level);
    
    match node {
        ASTNode::Number(n) => println!("{}Number({})", indent, n),
        ASTNode::Add(left, right) => {
            println!("{}Add", indent);
            print_ast(left, level + 1);
            print_ast(right, level + 1);
        },
        ASTNode::Sub(left, right) => {
            println!("{}Sub", indent);
            print_ast(left, level + 1);
            print_ast(right, level + 1);
        },
        ASTNode::Mul(left, right) => {
            println!("{}Mul", indent);
            print_ast(left, level + 1);
            print_ast(right, level + 1);
        },
        ASTNode::Div(left, right) => {
            println!("{}Div", indent);
            print_ast(left, level + 1);
            print_ast(right, level + 1);
        },
        ASTNode::LParen(expr) => {
            println!("{}LParen", indent);
            print_ast(expr, level + 1);
        },
        ASTNode::RParen(expr) => {
            println!("{}RParen", indent);
            print_ast(expr, level + 1);
        },
    }
}

fn sanitize_filename(input: &str) -> String {
    input.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}