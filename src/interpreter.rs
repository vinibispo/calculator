use std::{error::Error, fmt};

use crate::token::{Token, TokenKind};

pub struct Intepreter {
    pub text: String,
    pub pos: usize,
    pub current_token: Option<Token>,
    pub current_char: char,
}

#[derive(Debug)]
struct IntepreterError {
    pub message: String,
}

impl fmt::Display for IntepreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntepreterError: {}", self.message)
    }
}

impl Error for IntepreterError {}

impl Intepreter {
    pub fn new(text: String) -> Intepreter {
        let t = text.clone();
        Intepreter {
            text,
            pos: 0,
            current_token: None,
            current_char: t.chars().nth(0).unwrap(),
        }
    }

    fn term(&mut self) -> Result<i32, Box<dyn Error>> {
        let token = self.current_token.clone().unwrap();
        self.eat(TokenKind::Number);
        Ok(token.value.parse::<i32>().unwrap())
    }

    fn eat(&mut self, kind: TokenKind) {
        if self.current_token.clone().unwrap().kind == kind {
            self.current_token = self.get_next_token().unwrap();
        } else {
            panic!("Invalid syntax");
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() - 1 {
            self.current_char = '\0';
        } else {
            self.current_char = self.text.chars().nth(self.pos).unwrap();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char != '\0' && self.current_char.is_whitespace() {
            self.advance();
        }
    }

    fn integer(&mut self) -> i32 {
        let mut result = String::new();
        while self.current_char != '\0' && self.current_char.is_numeric() {
            result.push(self.current_char);
            self.advance();
        }
        result.parse::<i32>().unwrap()
    }

    fn get_next_token(&mut self) -> Result<Option<Token>, Box<dyn Error>> {
        while self.current_char != '\0' {
            if self.current_char.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if self.current_char.is_numeric() {
                return Ok(Some(Token::new(
                    TokenKind::Number,
                    self.integer().to_string(),
                )));
            }
            if self.current_char == '+' {
                self.advance();
                return Ok(Some(Token::new(TokenKind::Plus, "+".to_string())));
            }
            if self.current_char == '-' {
                self.advance();
                return Ok(Some(Token::new(TokenKind::Minus, "-".to_string())));
            }
            if self.current_char == '*' {
                self.advance();
                return Ok(Some(Token::new(TokenKind::Multiply, "*".to_string())));
            }
            if self.current_char == '/' {
                self.advance();
                return Ok(Some(Token::new(TokenKind::Divide, "/".to_string())));
            }
            return Err(Box::new(IntepreterError {
                message: "Invalid character".to_string(),
            }));
        }
        Ok(Some(Token::new(TokenKind::EOF, "".to_string())))
    }

    pub fn expr(&mut self) -> Result<i32, Box<dyn Error>> {
        self.current_token = self.get_next_token()?;
        let mut result = self.term()?;
        while self.current_token.clone().unwrap().kind != TokenKind::EOF {
            match self.current_token.clone().unwrap().kind {
                TokenKind::Plus => {
                    self.eat(TokenKind::Plus);
                    result += self.term()?;
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus);
                    result -= self.term()?;
                }
                TokenKind::Multiply => {
                    self.eat(TokenKind::Multiply);
                    result *= self.term()?;
                }
                TokenKind::Divide => {
                    self.eat(TokenKind::Divide);
                    result /= self.term()?;
                }
                _ => {
                    return Err(Box::new(IntepreterError {
                        message: "Invalid syntax".to_string(),
                    }))
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interpreter() {
        let interpreter = Intepreter::new("3".to_string());
        assert_eq!(interpreter.text, "3".to_string());
        assert_eq!(interpreter.pos, 0);
        assert_eq!(interpreter.current_token, None);
    }

    #[test]
    fn test_sum() {
        let mut interpreter = Intepreter::new("3+1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 4)
    }

    #[test]
    fn test_sum_with_many_digits() {
        let mut interpreter = Intepreter::new("123+456".to_string());
        assert_eq!(interpreter.expr().unwrap(), 579)
    }

    #[test]
    fn test_sum_with_spaces() {
        let mut interpreter = Intepreter::new(" 3 + 1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 4)
    }

    #[test]
    fn test_sum_with_many_spaces() {
        let mut interpreter = Intepreter::new("  3   +  1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 4)
    }

    #[test]
    fn test_subtraction() {
        let mut interpreter = Intepreter::new("3-1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 2)
    }

    #[test]
    fn test_subtraction_with_many_digits() {
        let mut interpreter = Intepreter::new("123-456".to_string());
        assert_eq!(interpreter.expr().unwrap(), -333)
    }

    #[test]
    fn test_subtraction_with_spaces() {
        let mut interpreter = Intepreter::new(" 3 - 1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 2)
    }

    #[test]
    fn test_subtraction_with_many_spaces() {
        let mut interpreter = Intepreter::new("  3   -  1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 2)
    }

    #[test]
    fn test_sum_and_subtraction() {
        let mut interpreter = Intepreter::new("3+1-1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 3)
    }

    #[test]
    fn test_sum_and_subtraction_with_many_digits() {
        let mut interpreter = Intepreter::new("123+456-1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 578)
    }

    #[test]
    fn test_sum_and_subtraction_with_spaces() {
        let mut interpreter = Intepreter::new(" 3 + 1 - 1".to_string());
        assert_eq!(interpreter.expr().unwrap(), 3)
    }

    #[test]
    fn test_multiplication() {
        let mut interpreter = Intepreter::new("3*2".to_string());
        assert_eq!(interpreter.expr().unwrap(), 6)
    }

    #[test]
    fn test_multiplication_with_many_digits() {
        let mut interpreter = Intepreter::new("123*456".to_string());
        assert_eq!(interpreter.expr().unwrap(), 56088)
    }

    #[test]
    fn test_multiplication_with_spaces() {
        let mut interpreter = Intepreter::new(" 3 * 2".to_string());
        assert_eq!(interpreter.expr().unwrap(), 6)
    }

    #[test]
    fn test_multiplication_with_many_spaces() {
        let mut interpreter = Intepreter::new("  3   *  2".to_string());
        assert_eq!(interpreter.expr().unwrap(), 6)
    }

    #[test]
    fn test_division() {
        let mut interpreter = Intepreter::new("3/2".to_string());
        assert_eq!(interpreter.expr().unwrap(), 1)
    }

    // #[test]
    // fn test_sum_and_multiplication() {
    //     let mut interpreter = Intepreter::new("3+1*2".to_string());
    //     assert_eq!(interpreter.expr().unwrap(), 5)
    // }
}
