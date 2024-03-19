use crate::{token::Token, visitor::Visitor};

use std::{cell::RefCell, rc::Rc, fmt};
pub trait Ast {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<i32, String>;
}

pub type AstNode = Rc<RefCell<dyn Ast>>;

pub struct BinaryOp {
    pub left: AstNode,
    pub right: AstNode,
    pub token: Token,
}

impl Ast for BinaryOp {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<i32, String> {
        visitor.visit_binary_op(self)
    }
}

impl BinaryOp {
    pub fn new(left: AstNode, right: AstNode, token: Token) -> AstNode {
        Rc::new(RefCell::new(BinaryOp { left, right, token }))
    }
}

pub struct Num {
    pub token: Token,
    pub value: i32,
}

impl Ast for Num {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<i32, String> {
        visitor.visit_num(self)
    }
}

impl Num {
    pub fn new(token: Token) -> AstNode {
        let value = token.value.parse().unwrap();
        Rc::new(RefCell::new(Num { token, value }))
    }
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Num({})", self.value)
    }
}

impl fmt::Debug for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BinaryOp({:?})", self.token)
    }
}
