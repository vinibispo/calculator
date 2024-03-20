use crate::token::Token;

#[derive(Debug, Clone)]
pub enum AstType {
    Integer(i32),
    Real(f64),
}

#[derive(Debug, Clone)]
pub enum AstNode {
    BinaryOp(Box<AstNode>, Box<AstNode>, Token),
    Num(AstType),
    UnaryOp(Box<AstNode>, Token),
    Var(Token),
    Assign(Box<AstNode>, Box<AstNode>, Token),
    Compound(Vec<AstNode>),
    NoOp,
    Program(String, Box<AstNode>),
    Block(Vec<AstNode>, Box<AstNode>),
    VarDecl(Box<AstNode>, Box<AstNode>),
    Type(Token),
}
