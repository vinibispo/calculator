use std::{error::Error, fmt};

use crate::token::{Token, TokenKind};
use crate::lexer::Lexer;

pub struct Interpreter<'a> {
    pub lexer: &'a mut Lexer,
    pub current_token: Option<Token>,
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
    pub fn new(lexer: &'a mut Lexer) -> Interpreter<'a> {
        let l = lexer;
        let token = l.get_next_token();
        Interpreter {
            lexer: l,
            current_token: token,
        }
    }

    fn eat(&mut self, kind: TokenKind) -> Result<(), InterpreterError> {
        if let Some(token) = self.current_token.clone() {
            if token.kind == kind {
                self.current_token = self.lexer.get_next_token();
                Ok(())
            } else {
                Err(InterpreterError { message: "Invalid syntax".to_string() })
            }
        } else {
            Err(InterpreterError { message: "Unexpected end of input".to_string() })
        }
    }

    fn factor(&mut self) -> Result<i32, InterpreterError>{
        if let Some(token) = self.current_token.clone() {
            match token.kind {
                TokenKind::Number => {
                    self.eat(TokenKind::Number)?;
                    Ok(token.value.parse::<i32>().unwrap())
                }
                TokenKind::LParen => {
                    self.eat(TokenKind::LParen)?;
                    let result = self.expr()?;
                    self.eat(TokenKind::RParen)?;
                    Ok(result)
                }
                _ => Err(InterpreterError { message: "Invalid syntax".to_string() })
            }
        }
        else {
            Err(InterpreterError { message: "Unexpected end of input".to_string() })
        }
    }

    fn term(&mut self) -> Result<i32, InterpreterError> {
        let mut result = self.factor()?;
        while let Some(token) = self.current_token.clone() {
            if token.kind == TokenKind::EOF {
                break;
            }
            if ![TokenKind::Multiply, TokenKind::Divide].contains(&token.kind) {
                break;
            }
            match token.kind {
                TokenKind::Multiply => {
                    self.eat(TokenKind::Multiply)?;
                    result *= self.factor()?;
                }
                TokenKind::Divide => {
                    self.eat(TokenKind::Divide)?;
                    result /= self.factor()?;
                }
                _ => break,
            }
        };
        Ok(result)
    }

     pub fn expr(&mut self) -> Result<i32, InterpreterError> {
        let mut result = self.term()?;
        while let Some(token) = self.current_token.clone() {
            if token.kind == TokenKind::EOF {
                break;
            }
            if ![TokenKind::Plus, TokenKind::Minus].contains(&token.kind) {
                break;
            }
            match token.kind {
                TokenKind::Plus => {
                    self.eat(TokenKind::Plus)?;
                    result += self.term()?;
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus)?;
                    result -= self.term()?;
                }
                _ => break,
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut lexer = Lexer::new("3+1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 4)
    }

    #[test]
    fn test_sum_with_many_digits() {
        let mut lexer = Lexer::new("123+456".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 579)
    }

    #[test]
    fn test_sum_with_spaces() {
        let mut lexer = Lexer::new(" 3 + 1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 4)
    }

    #[test]
    fn test_sum_with_many_spaces() {
        let mut lexer = Lexer::new("  3   +  1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 4)
    }

    #[test]
    fn test_subtraction() {
        let mut lexer = Lexer::new("3-1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 2)
    }

    #[test]
    fn test_subtraction_with_many_digits() {
        let mut lexer = Lexer::new("123-456".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), -333)
    }

    #[test]
    fn test_subtraction_with_spaces() {
        let mut lexer = Lexer::new(" 3 - 1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 2)
    }

    #[test]
    fn test_subtraction_with_many_spaces() {
        let mut lexer = Lexer::new("  3   -  1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 2)
    }

    #[test]
    fn test_sum_and_subtraction() {
        let mut lexer = Lexer::new("3+1-1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 3)
    }

    #[test]
    fn test_sum_and_subtraction_with_many_digits() {
        let mut lexer = Lexer::new("123+456-1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 578)
    }

    #[test]
    fn test_sum_and_subtraction_with_spaces() {
        let mut lexer = Lexer::new(" 3 + 1 - 1".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 3)
    }

    #[test]
    fn test_multiplication() {
        let mut lexer = Lexer::new("3*2".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 6)
    }

    #[test]
    fn test_multiplication_with_many_digits() {
        let mut lexer = Lexer::new("123*456".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 56088)
    }

    #[test]
    fn test_multiplication_with_spaces() {
        let mut lexer = Lexer::new(" 3 * 2".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 6)
    }

    #[test]
    fn test_multiplication_with_many_spaces() {
        let mut lexer = Lexer::new("  3   *  2".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 6)
    }

    #[test]
    fn test_division() {
        let mut lexer = Lexer::new("3/2".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 1)
    }

    #[test]
    fn test_sum_and_multiplication() {
        let mut lexer = Lexer::new("3+1*2".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 5)
    }

    #[test]
    fn test_sum_and_multiplication_with_many_digits() {
        let mut lexer = Lexer::new("123+456*2".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 1035)
    }

    #[test]
    fn test_sum_multiplication_and_subtraction_and_division_using_parentheses() {
        let mut lexer = Lexer::new("7 + 3 * (10 / (12 / (3 + 1) - 1))".to_string());
        let mut interpreter = Interpreter::new(&mut lexer);
        assert_eq!(interpreter.expr().unwrap(), 22)
    }
}
