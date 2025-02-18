use super::ast::ASTNode;   // super fait référence au module parent (parser)
use super::lexer::Token;   // pareil ici

/*
    Grammaire pour parse de simple expression arithmétique:
    expr   = term , { ("+" | "-") , term } ;  
    term   = factor , { ("*" | "/") , factor } ;  
    factor = number | "(" , expr , ")" | "-" , factor ;  


    EBNF :
        "" delimite un symbol terminal ex "+"
        , sépare des élements obligatoires 
        {} répetition 0 ou plusieurs fois
        () groupe des élements 


*/

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.parse_expr()
    }

    // expr   = term , { ("+" | "-") , term } ;  

    fn parse_expr(&mut self) -> Option<ASTNode> {
        let mut left = self.parse_term()?;  

        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = ASTNode::Add(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = ASTNode::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Some(left)
    }

    // term   = factor , { ("*" | "/") , factor } ;  

    fn parse_term(&mut self) -> Option<ASTNode> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Star => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = ASTNode::Mul(Box::new(left), Box::new(right));
                }
                Token::Slash => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = ASTNode::Div(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Some(left)
    }

    // factor = number | "(" , expr , ")" | "-" , factor ;  

    fn parse_factor(&mut self) -> Option<ASTNode> {
        match self.peek()? {
            Token::Number(n) => {
                let value = *n;
                self.consume();
                Some(ASTNode::Number(value))
            }
            Token::LParen => {
                self.consume();
                let expr = self.parse_expr()?;
                match self.consume()? {
                    Token::RParen => Some(expr),
                    _ => None,
                }
            }
            Token::Minus => {
                self.consume();
                let factor = self.parse_factor()?;
                Some(ASTNode::Sub(
                    Box::new(ASTNode::Number(0)),
                    Box::new(factor)
                ))
            }
            _ => None,
        }
    }
}



