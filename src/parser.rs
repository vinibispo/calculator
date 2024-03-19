use std::{error::Error, fmt};

use crate::ast::{AstNode, BinaryOp, Num};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub current_token: Option<Token>,
}

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IntepreterError: {}", self.message)
    }
}

impl Error for ParserError {}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        let l = lexer;
        let token = l.get_next_token();
        Parser {
            lexer: l,
            current_token: token,
        }
    }

    fn eat(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        if let Some(token) = self.current_token.clone() {
            if token.kind == kind {
                self.current_token = self.lexer.get_next_token();
                Ok(())
            } else {
                Err(ParserError {
                    message: "Invalid syntax".to_string(),
                })
            }
        } else {
            Err(ParserError {
                message: "Unexpected end of input".to_string(),
            })
        }
    }

    fn factor(&mut self) -> Result<AstNode, ParserError> {
        if let Some(token) = self.current_token.clone() {
            match token.kind {
                TokenKind::Number => {
                    self.eat(TokenKind::Number)?;
                    Ok(Num::new(token))
                }
                TokenKind::LParen => {
                    self.eat(TokenKind::LParen)?;
                    let result = self.expr()?;
                    self.eat(TokenKind::RParen)?;
                    Ok(result)
                }
                _ => Err(ParserError {
                    message: "Invalid syntax".to_string(),
                }),
            }
        } else {
            Err(ParserError {
                message: "Unexpected end of input".to_string(),
            })
        }
    }

    fn term(&mut self) -> Result<AstNode, ParserError> {
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
                    result = BinaryOp::new(result, self.factor()?, token);
                }
                TokenKind::Divide => {
                    self.eat(TokenKind::Divide)?;
                    result = BinaryOp::new(result, self.factor()?, token);
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn expr(&mut self) -> Result<AstNode, ParserError> {
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
                    result = BinaryOp::new(result, self.term()?, token);
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus)?;
                    result = BinaryOp::new(result, self.term()?, token);
                }
                _ => break,
            }
        }
        Ok(result)
    }

    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        self.expr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut lexer = Lexer::new("3 + 5 * 2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_with_parentheses() {
        let mut lexer = Lexer::new("(3 + 5) * 2".to_string());
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_with_invalid_syntax() {
        let mut lexer = Lexer::new("3 +".to_string());
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();
        assert!(result.is_err());
    }

}
