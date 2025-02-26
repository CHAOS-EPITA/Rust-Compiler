use std::fmt::Write;  
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

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        writeln!(&mut dot, "digraph AST {{").unwrap();
        writeln!(&mut dot, "    node [shape=box];").unwrap();
        
        let mut node_id = 0;
        self.write_dot(&mut dot, &mut node_id, None);
        
        writeln!(&mut dot, "}}").unwrap();
        dot
    }
    
    fn write_dot(&self, dot: &mut String, node_id: &mut usize, parent_id: Option<usize>) -> usize {
        let current_id = *node_id;
        *node_id += 1;
        
        let label = match self {
            ASTNode::Number(n) => format!("Number({})", n),
            ASTNode::Add(_, _) => "Add".to_string(),
            ASTNode::Mul(_, _) => "Mul".to_string(),
            ASTNode::Sub(_, _) => "Sub".to_string(),
            ASTNode::Div(_, _) => "Div".to_string(),
            ASTNode::LParen(_) => "LParen".to_string(),
            ASTNode::RParen(_) => "RParen".to_string(),
        };
        
        writeln!(dot, "    node{} [label=\"{}\"];", current_id, label).unwrap();
        
        // add edge from parent to current node
        if let Some(parent) = parent_id {
            writeln!(dot, "    node{} -> node{};", parent, current_id).unwrap();
        }
        
        match self {
            ASTNode::Add(left, right) | ASTNode::Mul(left, right) |
            ASTNode::Sub(left, right) | ASTNode::Div(left, right) => {
                left.write_dot(dot, node_id, Some(current_id));
                right.write_dot(dot, node_id, Some(current_id));
            },
            ASTNode::LParen(expr) | ASTNode::RParen(expr) => {
                expr.write_dot(dot, node_id, Some(current_id));
            },
            ASTNode::Number(_) => {}, 
        }
        
        current_id
    }
}