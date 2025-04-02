use logos::Logos;
use std::fmt;

/// Token représente tous les tokens possibles dans notre langage Rust simplifié
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // Mots-clés
    #[token("fn")]
    Fn,

    #[token("let")]
    Let,

    #[token("mut")]
    Mut,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("return")]
    Return,

    #[token("true")]
    True,

    #[token("false")]
    False,

    // Types de base
    #[token("i32")]
    I32,

    #[token("f64")]
    F64,

    #[token("bool")]
    Bool,

    #[token("str")]
    Str,

    // Délimiteurs
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("::")]
    DoubleColon,

    // Opérateurs
    #[token("=")]
    Assign,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("==")]
    Eq,

    #[token("!=")]
    NotEq,

    #[token("<")]
    Lt,

    #[token("<=")]
    LtEq,

    #[token(">")]
    Gt,

    #[token(">=")]
    GtEq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    // Littéraux
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    #[regex(r"[0-9]+")]
    IntLiteral,

    #[regex(r"[0-9]+\.[0-9]+")]
    FloatLiteral,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    // Commentaires et espaces
    #[regex(r"//.*", logos::skip)]
    Comment,

    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    MultiLineComment,

    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,

    // Token d'erreur
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident => write!(f, "identificateur"),
            Token::IntLiteral => write!(f, "littéral entier"),
            Token::FloatLiteral => write!(f, "littéral flottant"),
            Token::StringLiteral => write!(f, "littéral de chaîne"),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// TokenWithSpan représente un token avec sa position dans le code source
#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: (usize, usize),
    pub text: String,
}

impl TokenWithSpan {
    pub fn new(token: Token, span: (usize, usize), text: String) -> Self {
        Self { token, span, text }
    }
}
