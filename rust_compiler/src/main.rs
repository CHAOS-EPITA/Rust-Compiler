mod parser;

fn main() {
    let expressions = vec![
        "2 + 3",
        "2 * 3",
        "2 + 3 * 4",
        "2 * (3 + 4)",
    ];

    for expr in expressions {
        println!("\nAnalyse de l'expression : {}", expr);


        let tokens = parser::lexer::tokenize(expr);
        println!("Tokens générés : {:?}", tokens);
        

        let mut parser = parser::parser::Parser::new(tokens);
        
        match parser.parse() {
            Some(ast) => {
                println!("AST construit avec succès : {:?}", ast);
                println!("Résultat de l'évaluation : {}", ast.evaluate());
            },
            None => println!("Erreur : impossible de parser l'expression"),
        }
    }
}


#[cfg(test)]
mod tests;