use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i32),

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    Token::lexer(input).collect()
}