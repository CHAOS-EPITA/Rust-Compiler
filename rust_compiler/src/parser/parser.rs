use crate::parser::ast::ASTNode;
use crate::parser::lexer::Token;


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

    pub fn new (tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&self) -> Option<&Token>{
        self.expr()
    }


    // next token without consume
    fn peek (&mut self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self) -> Option<Token>{
        let token= self.tokens.get(self.position).cloned();
        self.position += 1;
        token

    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.parse_expr()
    }

    // parse first production rule of grammar 
    //  expr = term { ("+" | "-") term }

    fn parse_expr(&mut self) -> Option<ASTNode> {
        let mut left = match.self_parse_term(){ 
            Some(term) => term,
            None => return None,
        };

        while let Some(token) = self.peek(){
            match token {
                Token::Plus => {
                    self.consume();
                    let right = match self.parse_term(){
                        Some(term) => term,
                        None => return None,
                    };
                    left = ASTNode::Add(Box::new(left), Box::new(right));
                }
                Token::Minus=> {
                    self.consume();
                    let right = match self.parse_term(){
                        Some(term) => term,
                        None => return None,
                    }
                    left = ASTNode::Sub(Box::new(left), Box::new(right));
                }
                _ => break,

            }
        }
        Some(left) // return left or None

    }

    //  term = factor { ("*" | "/") factor }
    fn parse_term(&mut self) -> Option<ASTNode> {
        let mut left = match self.parse_factor() { 
            Some(factor) => factor,
            None => return None,
        };
        
        // while we have a token and it is a multiplication or division
        while let Some(token) = self.peek() {
            match token {
                Token::Mul => {
                    self.consume();
                    let right = match self.parse_factor() { 
                        Some(factor) => factor,
                        None => return None,
                    };
                    left = ASTNode::Mul(Box::new(left), Box::new(right));
                }
                Token::Div => {
                    self.consume();
                    let right = match self.parse_factor() { 
                        Some(factor) => factor,
                        None => return None,
                    }; 
                    left = ASTNode::Div(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Some(left)
    }



}