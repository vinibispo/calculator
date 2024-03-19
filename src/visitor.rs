use crate::ast::{BinaryOp, Num, UnaryOp};

pub trait Visitor {
    fn visit_binary_op(&mut self, binary_op: &BinaryOp) -> Result<i32, String>;
    fn visit_num(&mut self, num: &Num) -> Result<i32, String>;
    fn visit_unary_op(&mut self, _unary_op: &UnaryOp) -> Result<i32, String>;
}
