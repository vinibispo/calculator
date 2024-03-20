use std::{error::Error, fmt};

mod kind;

use crate::{
    ast::{AstNode, AstType},
    parser::Parser,
    token::{Token, TokenKind},
};
use kind::InterpreterType;

pub struct Interpreter<'a> {
    pub parser: &'a mut Parser<'a>,
    pub global_scope: std::collections::HashMap<String, InterpreterType>,
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
        Interpreter {
            parser,
            global_scope: std::collections::HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<f64, String> {
        let tree = self.parser.parse();
        match tree {
            Ok(tree) => match self.visit(tree) {
                Ok(value) => match value {
                    InterpreterType::Integer(value) => Ok(value as f64),
                    InterpreterType::Real(value) => Ok(value),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    fn visit_binary_op(
        &mut self,
        left: AstNode,
        right: AstNode,
        token: Token,
    ) -> Result<InterpreterType, String> {
        let left = self.visit(left)?;
        let right = self.visit(right)?;
        match token.kind {
            TokenKind::Plus => Ok(left + right),
            TokenKind::Minus => Ok(left - right),
            TokenKind::Multiply => Ok(left * right),
            TokenKind::FloatDivide => Ok(left / right),
            TokenKind::IntegerDivide => Ok(left.integer_div(right)),
            _ => Err("Invalid token".to_string()),
        }
    }

    fn visit_num(&mut self, num: InterpreterType) -> Result<InterpreterType, String> {
        Ok(num)
    }

    fn visit_unary_op(&mut self, node: AstNode, token: Token) -> Result<InterpreterType, String> {
        let node = self.visit(node)?;
        match token.kind {
            TokenKind::Plus => Ok(node),
            TokenKind::Minus => Ok(-node),
            _ => Err("Invalid token".to_string()),
        }
    }

    fn visit_compound(&mut self, nodes: Vec<AstNode>) -> Result<InterpreterType, String> {
        for node in nodes {
            self.visit(node)?;
        }
        Ok(InterpreterType::Real(0.0))
    }

    fn visit_assignment(
        &mut self,
        left: AstNode,
        right: AstNode,
        _token: Token,
    ) -> Result<InterpreterType, String> {
        let string = match left {
            AstNode::Var(token) => token.value,
            _ => return Err("Invalid token".to_string()),
        };
        let string = string.parse::<String>();
        let value = self.visit(right)?;
        self.global_scope.insert(string, value);
        Ok(value)
    }

    fn visit_var(&mut self, token: Token) -> Result<InterpreterType, String> {
        let string = token.value.parse::<String>();
        match self.global_scope.get(&string) {
            Some(value) => Ok(*value),
            None => Err("Variable not found".to_string()),
        }
    }

    fn visit_program(&mut self, _name: String, block: AstNode) -> Result<InterpreterType, String> {
        self.visit(block)
    }

    fn visit_block(
        &mut self,
        declarations: Vec<AstNode>,
        compound_statement: AstNode,
    ) -> Result<InterpreterType, String> {
        for declaration in declarations {
            self.visit(declaration)?;
        }
        self.visit(compound_statement)
    }

    pub fn visit(&mut self, node: AstNode) -> Result<InterpreterType, String> {
        match node {
            AstNode::Program(name, block) => self.visit_program(name, *block),
            AstNode::Block(declarations, compound_statement) => {
                self.visit_block(declarations, *compound_statement)
            }
            AstNode::BinaryOp(left, right, token) => self.visit_binary_op(*left, *right, token),
            AstNode::Num(num) => {
                let num = match num {
                    AstType::Integer(value) => InterpreterType::Integer(value),
                    AstType::Real(value) => InterpreterType::Real(value),
                    // _ => return Err("Invalid token".to_string()),
                };
                self.visit_num(num)
            }
            AstNode::UnaryOp(node, token) => self.visit_unary_op(*node, token),
            AstNode::Compound(nodes) => self.visit_compound(nodes),
            AstNode::Assign(left, right, token) => self.visit_assignment(*left, *right, token),
            AstNode::Var(token) => self.visit_var(token),
            _ => Ok(InterpreterType::Real(0.0)),
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
        assert_eq!(interpreter.interpret().unwrap(), 4.0)
    }

    #[test]
    fn test_sum_with_many_digits() {
        let mut lexer = Lexer::new("123+456".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 579.0)
    }

    #[test]
    fn test_sum_with_spaces() {
        let mut lexer = Lexer::new(" 3 + 1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 4.0)
    }

    #[test]
    fn test_sum_with_many_spaces() {
        let mut lexer = Lexer::new("  3   +  1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 4.0)
    }

    #[test]
    fn test_subtraction() {
        let mut lexer = Lexer::new("3-1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 2.0)
    }

    #[test]
    fn test_subtraction_with_many_digits() {
        let mut lexer = Lexer::new("123-456".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), -333.0)
    }

    #[test]
    fn test_subtraction_with_spaces() {
        let mut lexer = Lexer::new(" 3 - 1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 2.0)
    }

    #[test]
    fn test_subtraction_with_many_spaces() {
        let mut lexer = Lexer::new("  3   -  1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 2.0)
    }

    #[test]
    fn test_sum_and_subtraction() {
        let mut lexer = Lexer::new("3+1-1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 3.0)
    }

    #[test]
    fn test_sum_and_subtraction_with_many_digits() {
        let mut lexer = Lexer::new("123+456-1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 578.0)
    }

    #[test]
    fn test_sum_and_subtraction_with_spaces() {
        let mut lexer = Lexer::new(" 3 + 1 - 1".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 3.0)
    }

    #[test]
    fn test_multiplication() {
        let mut lexer = Lexer::new("3*2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 6.0)
    }

    #[test]
    fn test_multiplication_with_many_digits() {
        let mut lexer = Lexer::new("123*456".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 56_088.0)
    }

    #[test]
    fn test_multiplication_with_spaces() {
        let mut lexer = Lexer::new(" 3 * 2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 6.0)
    }

    #[test]
    fn test_multiplication_with_many_spaces() {
        let mut lexer = Lexer::new("  3   *  2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 6.0)
    }

    #[test]
    fn test_division() {
        let mut lexer = Lexer::new("3 DIV 2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 1.0)
    }

    #[test]
    fn test_sum_and_multiplication() {
        let mut lexer = Lexer::new("3+1*2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 5.0)
    }

    #[test]
    fn test_sum_and_multiplication_with_many_digits() {
        let mut lexer = Lexer::new("123+456*2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 1_035.0)
    }

    #[test]
    fn test_sum_multiplication_and_subtraction_and_division_using_parentheses() {
        let mut lexer = Lexer::new("7 + 3 * (10.0 / (12 / (3 + 1) - 1))".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 22.0)
    }

    #[test]
    fn test_unary_operations() {
        let mut lexer = Lexer::new("5 - - - + - (3 + 4) - +2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 10.0)
    }

    #[test]
    fn test_assignment() {
        let mut lexer = Lexer::new("BEGIN a := 5; END.".to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 0.0);
        assert_eq!(interpreter.global_scope.get("a").unwrap().from::<i32>(), 5)
    }

    #[test]
    fn test_with_program() {
        let string = "
            PROGRAM Part10;
            VAR
               x, y, z : INTEGER;
            BEGIN {Part10}
               BEGIN
                   x := 5;
                   y := x + 10;
                   z := y DIV 3;
               END;
            END.  "
            .to_string();
        let mut lexer = Lexer::new(string);
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        assert_eq!(interpreter.interpret().unwrap(), 0.0);
        assert_eq!(interpreter.global_scope.get("x").unwrap().from::<i32>(), 5);
        assert_eq!(interpreter.global_scope.get("y").unwrap().from::<i32>(), 15);
        assert_eq!(interpreter.global_scope.get("z").unwrap().from::<i32>(), 5)
    }
}
