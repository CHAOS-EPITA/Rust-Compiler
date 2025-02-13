
#[derive(Debug)] 
pub enum ASTNode {
    Number(i32),
    Add(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
    LParen(Box<ASTNode>),
    RParen(Box<ASTNode>),
    

}
