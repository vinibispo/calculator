use std::{error::Error, fmt};

use crate::{
    ast::AstNode,
    parser::Parser,
    token::{Token, TokenKind},
};

pub struct Interpreter<'a> {
    pub parser: &'a mut Parser<'a>,
    pub global_scope: std::collections::HashMap<String, i32>,
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

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Interpreter { parser, global_scope: std::collections::HashMap::new() }
    }

    pub fn interpret(&mut self) -> Result<i32, String> {
        let tree = self.parser.parse();
        match tree {
            Ok(tree) => self.visit(tree),
            Err(e) => Err(e.to_string()),
        }
    }

    fn visit_binary_op(
        &mut self,
        left: AstNode,
        right: AstNode,
        token: Token,
    ) -> Result<i32, String> {
        let left = self.visit(left)?;
        let right = self.visit(right)?;
        match token.kind {
            TokenKind::Plus => Ok(left + right),
            TokenKind::Minus => Ok(left - right),
            TokenKind::Multiply => Ok(left * right),
            TokenKind::Divide => Ok(left / right),
            _ => Err("Invalid token".to_string()),
        }
    }

    fn visit_num(&mut self, num: i32) -> Result<i32, String> {
        Ok(num)
    }

    fn visit_unary_op(&mut self, node: AstNode, token: Token) -> Result<i32, String> {
        let node = self.visit(node)?;
        match token.kind {
            TokenKind::Plus => Ok(node),
            TokenKind::Minus => Ok(-node),
            _ => Err("Invalid token".to_string()),
        }
    }

    fn visit_compound(&mut self, nodes: Vec<AstNode>) -> Result<i32, String> {
        for node in nodes {
            self.visit(node)?;
        }
        Ok(0)
    }

    fn visit_assignment(&mut self, left: AstNode, right: AstNode, _token: Token) -> Result<i32, String> {
        let string = match left {
            AstNode::Var(token) => token.value,
            _ => return Err("Invalid token".to_string()),
        };
        let value = self.visit(right)?;
        self.global_scope.insert(string, value);
        Ok(value)
    }

    fn visit_var(&mut self, token: Token) -> Result<i32, String> {
        let string = token.value;
        match self.global_scope.get(&string) {
            Some(value) => Ok(*value),
            None => Err("Variable not found".to_string()),
        }
    }

    pub fn visit(&mut self, node: AstNode) -> Result<i32, String> {
        match node {
            AstNode::BinaryOp(left, right, token) => self.visit_binary_op(*left, *right, token),
            AstNode::Num(num) => self.visit_num(num),
            AstNode::UnaryOp(node, token) => self.visit_unary_op(*node, token),
            AstNode::Compound(nodes) => self.visit_compound(nodes),
            AstNode::Assign(left, right, token) => self.visit_assignment(*left, *right, token),
            AstNode::Var(token) => self.visit_var(token),
            AstNode::NoOp => Ok(0),
        }
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

    #[test]
    fn test_assignment() {
        let mut lexer = Lexer::new("BEGIN a := 5; END.".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 0);
        assert_eq!(interpreter.global_scope.get("a").unwrap(), &5)
    }
}
