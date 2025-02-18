#[derive(Debug, PartialEq)] 
pub enum ASTNode {
    Number(i32),
    Add(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
    LParen(Box<ASTNode>),
    RParen(Box<ASTNode>),
}


impl ASTNode {
    pub fn evaluate(&self) -> i32 {
        match self {
            ASTNode::Number(n) => *n,
            ASTNode::Add(left, right) => left.evaluate() + right.evaluate(),
            ASTNode::Mul(left, right) => left.evaluate() * right.evaluate(),
            ASTNode::Sub(left, right) => left.evaluate() - right.evaluate(),
            ASTNode::Div(left, right) => left.evaluate() / right.evaluate(),
            ASTNode::LParen(expr) => expr.evaluate(),
            ASTNode::RParen(expr) => expr.evaluate(),
        }
    }
}