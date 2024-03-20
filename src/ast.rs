use crate::token::Token;

#[derive(Debug)]
pub enum AstNode {
    BinaryOp(Box<AstNode>, Box<AstNode>, Token),
    Num(i32),
    UnaryOp(Box<AstNode>, Token),
    Var(Token),
    Assign(Box<AstNode>, Box<AstNode>, Token),
    Compound(Vec<AstNode>),
    NoOp,
}
