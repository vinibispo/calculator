use std::{error::Error, fmt};

use crate::{
    ast::{AstNode, BinaryOp, Num, UnaryOp},
    parser::Parser,
    token::TokenKind,
    visitor::Visitor,
};

pub struct Interpreter<'a> {
    pub parser: &'a mut Parser<'a>,
}

#[derive(Debug)]
pub struct InterpreterError {
    pub message: String,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntepreterError: {}", self.message)
    }
}

impl Error for InterpreterError {}

impl Visitor for Interpreter<'_> {
    fn visit_binary_op(&mut self, binary_op: &BinaryOp) -> Result<i32, String> {
        let left = self.visit(binary_op.left.clone())?;
        let right = self.visit(binary_op.right.clone())?;
        match binary_op.token.kind {
            TokenKind::Plus => Ok(left + right),
            TokenKind::Minus => Ok(left - right),
            TokenKind::Multiply => Ok(left * right),
            TokenKind::Divide => Ok(left / right),
            _ => Err("Invalid operator".to_string()),
        }
    }

    fn visit_num(&mut self, num: &Num) -> Result<i32, String> {
        Ok(num.value)
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOp) -> Result<i32, String> {
        let expr = self.visit(unary_op.expr.clone())?;
        match unary_op.token.kind {
            TokenKind::Plus => Ok(expr),
            TokenKind::Minus => Ok(-expr),
            _ => Err("Invalid operator".to_string()),
        }
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Interpreter { parser }
    }

    pub fn interpret(&mut self) -> Result<i32, String> {
        let tree = self.parser.parse();
        match tree {
            Ok(tree) => self.visit(tree),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn visit(&mut self, node: AstNode) -> Result<i32, String> {
        let node = node.borrow();
        node.accept(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_sum() {
        let mut lexer = Lexer::new("3+1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 4)
    }

    #[test]
    fn test_sum_with_many_digits() {
        let mut lexer = Lexer::new("123+456".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 579)
    }

    #[test]
    fn test_sum_with_spaces() {
        let mut lexer = Lexer::new(" 3 + 1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 4)
    }

    #[test]
    fn test_sum_with_many_spaces() {
        let mut lexer = Lexer::new("  3   +  1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 4)
    }

    #[test]
    fn test_subtraction() {
        let mut lexer = Lexer::new("3-1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 2)
    }

    #[test]
    fn test_subtraction_with_many_digits() {
        let mut lexer = Lexer::new("123-456".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), -333)
    }

    #[test]
    fn test_subtraction_with_spaces() {
        let mut lexer = Lexer::new(" 3 - 1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 2)
    }

    #[test]
    fn test_subtraction_with_many_spaces() {
        let mut lexer = Lexer::new("  3   -  1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 2)
    }

    #[test]
    fn test_sum_and_subtraction() {
        let mut lexer = Lexer::new("3+1-1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 3)
    }

    #[test]
    fn test_sum_and_subtraction_with_many_digits() {
        let mut lexer = Lexer::new("123+456-1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 578)
    }

    #[test]
    fn test_sum_and_subtraction_with_spaces() {
        let mut lexer = Lexer::new(" 3 + 1 - 1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 3)
    }

    #[test]
    fn test_multiplication() {
        let mut lexer = Lexer::new("3*2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 6)
    }

    #[test]
    fn test_multiplication_with_many_digits() {
        let mut lexer = Lexer::new("123*456".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 56088)
    }

    #[test]
    fn test_multiplication_with_spaces() {
        let mut lexer = Lexer::new(" 3 * 2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 6)
    }

    #[test]
    fn test_multiplication_with_many_spaces() {
        let mut lexer = Lexer::new("  3   *  2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 6)
    }

    #[test]
    fn test_division() {
        let mut lexer = Lexer::new("3/2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 1)
    }

    #[test]
    fn test_sum_and_multiplication() {
        let mut lexer = Lexer::new("3+1*2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 5)
    }

    #[test]
    fn test_sum_and_multiplication_with_many_digits() {
        let mut lexer = Lexer::new("123+456*2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 1035)
    }

    #[test]
    fn test_sum_multiplication_and_subtraction_and_division_using_parentheses() {
        let mut lexer = Lexer::new("7 + 3 * (10 / (12 / (3 + 1) - 1))".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 22)
    }

    #[test]
    fn test_unary_operations() {
        let mut lexer = Lexer::new("5 - - - + - (3 + 4) - +2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 10)
    }
}
