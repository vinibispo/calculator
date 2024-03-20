use std::{error::Error, fmt};

use crate::ast::AstNode;
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

    fn program(&mut self) -> Result<AstNode, ParserError> {
        let node: AstNode;
        match self.current_token.clone() {
            Some(token) => match token.kind {
                TokenKind::Begin => {
                    node = self.compound_statement()?;
                    self.eat(TokenKind::Dot)?;
                }
                _ => node = self.expr()?,
            },
            None => return Err(ParserError {
                message: "Unexpected end of input".to_string(),
            }),
        };
        self.eat(TokenKind::EOF)?;
        Ok(node)
    }

    fn compound_statement(&mut self) -> Result<AstNode, ParserError> {
        self.eat(TokenKind::Begin)?;
        let nodes = self.statement_list()?;
        self.eat(TokenKind::End)?;
        let root = AstNode::Compound(nodes);
        Ok(root)
    }

    fn statement_list(&mut self) -> Result<Vec<AstNode>, ParserError> {
        let node = self.statement()?;
        let mut results = vec![node];
        while let Some(token) = self.current_token.clone() {
            if token.kind == TokenKind::Semi {
                self.eat(TokenKind::Semi)?;
                results.push(self.statement()?);
            } else {
                break;
            }
        }
        Ok(results)
    }

    fn statement(&mut self) -> Result<AstNode, ParserError> {
        if let Some(token) = self.current_token.clone() {
            println!("{:?}", token);
            match token.kind {
                TokenKind::Begin => self.compound_statement(),
                TokenKind::Identifier => self.assignment_statement(),
                _ => self.empty(),
            }
        } else {
            Err(ParserError {
                message: "Unexpected end of input".to_string(),
            })
        }
    }

    fn empty(&mut self) -> Result<AstNode, ParserError> {
        Ok(AstNode::NoOp)
    }

    fn assignment_statement(&mut self) -> Result<AstNode, ParserError> {
        let left = self.variable()?;
        let token = self.current_token.clone().unwrap();
        self.eat(TokenKind::Assign)?;
        let right = self.expr()?;
        Ok(AstNode::Assign(Box::new(left), Box::new(right), token))
    }

    fn variable(&mut self) -> Result<AstNode, ParserError> {
        if let Some(token) = self.current_token.clone() {
            if token.kind == TokenKind::Identifier {
                self.eat(TokenKind::Identifier)?;
                Ok(AstNode::Var(token))
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
                TokenKind::Plus => {
                    self.eat(TokenKind::Plus)?;
                    Ok(AstNode::UnaryOp(Box::new(self.factor()?), token))
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus)?;
                    Ok(AstNode::UnaryOp(Box::new(self.factor()?), token))
                }
                TokenKind::Number => {
                    self.eat(TokenKind::Number)?;
                    Ok(AstNode::Num(token.value.parse().unwrap()))
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
                    result = AstNode::BinaryOp(Box::new(result), Box::new(self.factor()?), token);
                }
                TokenKind::Divide => {
                    self.eat(TokenKind::Divide)?;
                    result = AstNode::BinaryOp(Box::new(result), Box::new(self.factor()?), token);
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
                    result = AstNode::BinaryOp(Box::new(result), Box::new(self.term()?), token);
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus)?;
                    result = AstNode::BinaryOp(Box::new(result), Box::new(self.term()?), token);
                }
                _ => break,
            }
        }
        Ok(result)
    }

    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        let node = self.program()?;
        if let Some(token) = self.current_token.clone() {
            if token.kind != TokenKind::EOF {
                return Err(ParserError {
                    message: "Invalid syntax".to_string(),
                });
            }
        }
        Ok(node)
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

    #[test]
    fn test_parser_with_unary_operator() {
        let mut lexer = Lexer::new("-3 + 5".to_string());
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_with_assignment() {
        let mut lexer = Lexer::new("BEGIN a := 5; END.".to_string());
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
