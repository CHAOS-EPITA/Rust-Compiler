use std::fmt;
use crate::common::Span;

/// Type représente les types primitifs du langage
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    F64,
    Bool,
    Str,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::F64 => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::Str => write!(f, "str"),
        }
    }
}

/// Node est un trait pour tous les nœuds de l'AST
pub trait Node {
    fn span(&self) -> Span;
}

/// Programme représente un programme complet
#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
    pub span: Span,
}

impl Node for Program {
    fn span(&self) -> Span {
        self.span
    }
}

/// Déclaration représente une déclaration (fonction ou variable)
#[derive(Debug, Clone)]
pub enum Declaration {
    Function(FunctionDecl),
    Variable(VariableDecl),
}

impl Node for Declaration {
    fn span(&self) -> Span {
        match self {
            Declaration::Function(f) => f.span(),
            Declaration::Variable(v) => v.span(),
        }
    }
}

/// FunctionDecl représente une déclaration de fonction
#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}

impl Node for FunctionDecl {
    fn span(&self) -> Span {
        self.span
    }
}

/// Paramètre représente un paramètre de fonction
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
    pub span: Span,
}

impl Node for Parameter {
    fn span(&self) -> Span {
        self.span
    }
}

/// VariableDecl représente une déclaration de variable
#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub typ: Option<Type>,
    pub is_mutable: bool,
    pub initializer: Expr,
    pub span: Span,
}

impl Node for VariableDecl {
    fn span(&self) -> Span {
        self.span
    }
}

/// Instruction représente une instruction dans un bloc
#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(ExprStmt),
    Declaration(Declaration),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Return(ReturnStmt),
    Block(Block),
}

impl Node for Stmt {
    fn span(&self) -> Span {
        match self {
            Stmt::Expr(e) => e.span(),
            Stmt::Declaration(d) => d.span(),
            Stmt::If(i) => i.span(),
            Stmt::While(w) => w.span(),
            Stmt::For(f) => f.span(),
            Stmt::Return(r) => r.span(),
            Stmt::Block(b) => b.span(),
        }
    }
}

/// ExprStmt représente une expression utilisée comme instruction
#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
    pub span: Span,
}

impl Node for ExprStmt {
    fn span(&self) -> Span {
        self.span
    }
}

/// IfStmt représente une instruction if-else
#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Block,
    pub else_branch: Option<Box<Stmt>>, // Peut être un Block ou un autre IfStmt
    pub span: Span,
}

impl Node for IfStmt {
    fn span(&self) -> Span {
        self.span
    }
}

/// WhileStmt représente une boucle while
#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
    pub span: Span,
}

impl Node for WhileStmt {
    fn span(&self) -> Span {
        self.span
    }
}

/// ForStmt représente une boucle for
#[derive(Debug, Clone)]
pub struct ForStmt {
    pub variable: String,
    pub iterator: Expr,
    pub body: Block,
    pub span: Span,
}

impl Node for ForStmt {
    fn span(&self) -> Span {
        self.span
    }
}

/// ReturnStmt représente une instruction de retour
#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
    pub span: Span,
}

impl Node for ReturnStmt {
    fn span(&self) -> Span {
        self.span
    }
}

/// Block représente un bloc de code
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

impl Node for Block {
    fn span(&self) -> Span {
        self.span
    }
}

/// Expr représente une expression
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(Literal),
    Identifier(Identifier),
    Call(FunctionCall),
    Assign(AssignExpr),
}

impl Node for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Binary(b) => b.span(),
            Expr::Unary(u) => u.span(),
            Expr::Literal(l) => l.span(),
            Expr::Identifier(i) => i.span(),
            Expr::Call(c) => c.span(),
            Expr::Assign(a) => a.span(),
        }
    }
}

/// BinaryOp représente un opérateur binaire
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Mod,     // %
    Eq,      // ==
    NotEq,   // !=
    Lt,      // <
    LtEq,    // <=
    Gt,      // >
    GtEq,    // >=
    And,     // &&
    Or,      // ||
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::NotEq => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::LtEq => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::GtEq => write!(f, ">="),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
        }
    }
}

/// BinaryExpr représente une expression binaire
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
    pub span: Span,
}

impl Node for BinaryExpr {
    fn span(&self) -> Span {
        self.span
    }
}

/// UnaryOp représente un opérateur unaire
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg, // -
    Pos, // +
    Not, // !
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Pos => write!(f, "+"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

/// UnaryExpr représente une expression unaire
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
    pub span: Span,
}

impl Node for UnaryExpr {
    fn span(&self) -> Span {
        self.span
    }
}

/// AssignExpr représente une expression d'affectation
#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
    pub span: Span,
}

impl Node for AssignExpr {
    fn span(&self) -> Span {
        self.span
    }
}

/// Literal représente une valeur littérale
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32, Span),
    Float(f64, Span),
    Bool(bool, Span),
    String(String, Span),
}

impl Node for Literal {
    fn span(&self) -> Span {
        match self {
            Literal::Int(_, s) => *s,
            Literal::Float(_, s) => *s,
            Literal::Bool(_, s) => *s,
            Literal::String(_, s) => *s,
        }
    }
}

/// Identifier représente un identifiant
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Node for Identifier {
    fn span(&self) -> Span {
        self.span
    }
}

/// FunctionCall représente un appel de fonction
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
    pub span: Span,
}

impl Node for FunctionCall {
    fn span(&self) -> Span {
        self.span
    }
}
