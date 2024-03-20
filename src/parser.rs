use std::{error::Error, fmt};

use crate::ast::{AstNode, AstType};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub current_token: Option<Token>,
}

#[derive(Debug, Clone)]
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
        // program: PROGRAM variable SEMI block DOT
        // | BEGIN statement_list END
        let node: AstNode;
        match self.current_token.clone() {
            Some(token) => match token.kind {
                TokenKind::Program => {
                    self.eat(TokenKind::Program)?;
                    let var_node = match self.variable()? {
                        AstNode::Var(var_node) => var_node.value.parse::<String>(),
                        _ => {
                            return Err(ParserError {
                                message: "Invalid syntax".to_string(),
                            });
                        }
                    };
                    self.eat(TokenKind::Semi)?;
                    let block_node = self.block()?;
                    node = AstNode::Program(var_node, Box::new(block_node));
                    self.eat(TokenKind::Dot)?;
                }
                TokenKind::Begin => {
                    node = self.compound_statement()?;
                    self.eat(TokenKind::Dot)?;
                }
                _ => node = self.expr()?,
            },
            None => {
                return Err(ParserError {
                    message: "Unexpected end of input".to_string(),
                })
            }
        };
        self.eat(TokenKind::EOF)?;
        Ok(node)
    }

    fn block(&mut self) -> Result<AstNode, ParserError> {
        // block : declarations compound_statement
        let declarations = self.declarations()?;
        let compound_statement = self.compound_statement()?;
        Ok(AstNode::Block(declarations, Box::new(compound_statement)))
    }

    fn declarations(&mut self) -> Result<Vec<AstNode>, ParserError> {
        // declarations : VAR (variable_declaration SEMI)+
        // | empty
        let mut declarations = vec![];
        if let Some(token) = self.current_token.clone() {
            if token.kind == TokenKind::Var {
                self.eat(TokenKind::Var)?;
                while let Some(token) = self.current_token.clone() {
                    if token.kind == TokenKind::Identifier {
                        declarations.append(&mut self.variable_declaration()?);
                        self.eat(TokenKind::Semi)?;
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(declarations)
    }

    fn variable_declaration(&mut self) -> Result<Vec<AstNode>, ParserError> {
        // variable_declaration : ID (COMMA ID)* COLON type_spec
        let mut var_nodes = vec![AstNode::Var(self.current_token.clone().unwrap())];
        self.eat(TokenKind::Identifier)?;
        while let Some(token) = self.current_token.clone() {
            match token.kind {
                TokenKind::Comma => {
                    self.eat(TokenKind::Comma)?;
                    var_nodes.push(AstNode::Var(self.current_token.clone().unwrap()));
                    self.eat(TokenKind::Identifier)?;
                }
                _ => break,
            }
        }
        self.eat(TokenKind::Colon)?;
        let type_node = self.type_spec()?;
        let mut declarations = vec![];
        for var_node in var_nodes {
            let type_n = type_node.clone();
            declarations.push(AstNode::VarDecl(Box::new(var_node), Box::new(type_n)));
        }
        Ok(declarations)
    }

    fn type_spec(&mut self) -> Result<AstNode, ParserError> {
        // type_spec : INTEGER
        // | REAL
        let token = self.current_token.clone().unwrap();
        match token.kind {
            TokenKind::Integer => {
                self.eat(TokenKind::Integer)?;
                Ok(AstNode::Type(token))
            }
            TokenKind::Real => {
                self.eat(TokenKind::Real)?;
                Ok(AstNode::Type(token))
            }
            _ => Err(ParserError {
                message: "Invalid syntax".to_string(),
            }),
        }
    }

    fn compound_statement(&mut self) -> Result<AstNode, ParserError> {
        // compound_statement: BEGIN statement_list END
        self.eat(TokenKind::Begin)?;
        let nodes = self.statement_list()?;
        self.eat(TokenKind::End)?;
        let root = AstNode::Compound(nodes);
        Ok(root)
    }

    fn statement_list(&mut self) -> Result<Vec<AstNode>, ParserError> {
        // statement_list : statement
        // | statement SEMI statement_list
        let node = self.statement()?;
        let mut results = vec![node];
        while let Some(token) = self.current_token.clone() {
            if token.kind == TokenKind::Semi {
                self.eat(TokenKind::Semi)?;
                let other_node = self.statement()?;
                results.push(other_node);
            } else {
                break;
            }
        }
        Ok(results)
    }

    fn statement(&mut self) -> Result<AstNode, ParserError> {
        if let Some(token) = self.current_token.clone() {
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
        // An empty production
        Ok(AstNode::NoOp)
    }

    fn assignment_statement(&mut self) -> Result<AstNode, ParserError> {
        // assignment_statement : variable ASSIGN expr

        let left = self.variable()?;
        let token = self.current_token.clone().unwrap();
        self.eat(TokenKind::Assign)?;
        let right = self.expr()?;
        Ok(AstNode::Assign(Box::new(left), Box::new(right), token))
    }

    fn variable(&mut self) -> Result<AstNode, ParserError> {
        // variable : ID
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
                TokenKind::Integer => {
                    self.eat(TokenKind::Integer)?;
                    Ok(AstNode::Num(AstType::Integer(token.value.parse::<i32>())))
                }
                TokenKind::Real => {
                    self.eat(TokenKind::Real)?;
                    Ok(AstNode::Num(AstType::Real(token.value.parse::<f64>())))
                }
                TokenKind::LParen => {
                    self.eat(TokenKind::LParen)?;
                    let result = self.expr()?;
                    self.eat(TokenKind::RParen)?;
                    Ok(result)
                }
                _ => self.variable(),
            }
        } else {
            Err(ParserError {
                message: "Unexpected end of input".to_string(),
            })
        }
    }

    fn term(&mut self) -> Result<AstNode, ParserError> {
        // term : factor ((MUL | DIV) factor)*
        let mut node = self.factor()?;
        while let Some(token) = self.current_token.clone() {
            match token.kind {
                TokenKind::Multiply => {
                    self.eat(TokenKind::Multiply)?;
                    node = AstNode::BinaryOp(Box::new(node), Box::new(self.factor()?), token);
                }
                TokenKind::FloatDivide => {
                    self.eat(TokenKind::FloatDivide)?;
                    node = AstNode::BinaryOp(Box::new(node), Box::new(self.factor()?), token);
                }
                TokenKind::IntegerDivide => {
                    self.eat(TokenKind::IntegerDivide)?;
                    node = AstNode::BinaryOp(Box::new(node), Box::new(self.factor()?), token);
                }
                _ => break,
            }
        }
        Ok(node)
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

    #[test]
    fn test_parser_with_program() {
        let mut lexer = Lexer::new(
            "PROGRAM Part10; VAR number : INTEGER; a, b : INTEGER; y : REAL; BEGIN END."
                .to_string(),
        );
        let mut parser = Parser::new(&mut lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }
    #[test]
    fn test_parse_with_program_and_more_declarations() {
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
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
