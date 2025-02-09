use logos::Logos;


#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i32),

    #[token("EOF")]
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(token);
    }
    tokens
}

/*

    CECI EST UN LEXER DE TEST EN ATTENDANT QUE MARCUS FINISSE SON LEXER
    (JE TE DOMINE D'AILLEURS)


*/