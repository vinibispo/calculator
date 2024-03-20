use crate::token::{Token, TokenKind, TokenValue, RESERVED_KEYWORDS};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub text: String,
    pub pos: usize,
    pub current_char: char,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let t = text.clone();
        Lexer {
            text,
            pos: 0,
            current_char: t.chars().nth(0).unwrap(),
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

    fn number(&mut self) -> Token {
        let mut result = String::new();
        while self.current_char != '\0' && self.current_char.is_numeric() {
            result.push(self.current_char);
            self.advance();
        }

        if self.current_char == '.' {
            result.push(self.current_char);
            self.advance();
            while self.current_char != '\0' && self.current_char.is_numeric() {
                result.push(self.current_char);
                self.advance();
            }
            Token::new(
                TokenKind::Real,
                TokenValue::Real(result.parse::<f64>().unwrap()),
            )
        } else {
            Token::new(
                TokenKind::Integer,
                TokenValue::Int(result.parse::<i32>().unwrap()),
            )
        }
    }

    fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos > self.text.len() - 1 {
            None
        } else {
            Some(self.text.chars().nth(peek_pos).unwrap())
        }
    }

    fn id(&mut self) -> Token {
        let mut result = String::new();
        while self.current_char != '\0' && self.current_char.is_alphanumeric() {
            result.push(self.current_char);
            self.advance();
        }
        for (kind, value) in RESERVED_KEYWORDS.iter() {
            if result == *value {
                let kind = kind.clone();
                return Token::new(kind, TokenValue::Str(result));
            }
        }
        Token::new(TokenKind::Identifier, TokenValue::Str(result))
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while self.current_char != '\0' {
            if self.current_char.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if self.current_char == '{' {
                self.advance();
                self.skip_comment();
                continue;
            }

            if self.current_char.is_alphabetic() {
                return Some(self.id());
            }

            if self.current_char.is_numeric() {
                return Some(self.number());
            }

            match self.current_char {
                '+' => {
                    self.advance();
                    return Some(Token::new(
                        TokenKind::Plus,
                        TokenValue::Str("+".to_string()),
                    ));
                }
                '-' => {
                    self.advance();
                    return Some(Token::new(
                        TokenKind::Minus,
                        TokenValue::Str("-".to_string()),
                    ));
                }
                '*' => {
                    self.advance();
                    return Some(Token::new(
                        TokenKind::Multiply,
                        TokenValue::Str("*".to_string()),
                    ));
                }
                '/' => {
                    self.advance();
                    return Some(Token::new(
                        TokenKind::FloatDivide,
                        TokenValue::Str("/".to_string()),
                    ));
                }
                '(' => {
                    self.advance();
                    return Some(Token::new(
                        TokenKind::LParen,
                        TokenValue::Str("(".to_string()),
                    ));
                }
                ')' => {
                    self.advance();
                    return Some(Token::new(
                        TokenKind::RParen,
                        TokenValue::Str(")".to_string()),
                    ));
                }
                ':' if self.peek() == Some('=') => {
                    self.advance();
                    self.advance();
                    let symbol = TokenValue::Str(":=".to_string());
                    return Some(Token::new(TokenKind::Assign, symbol));
                }
                ':' => {
                    self.advance();
                    let symbol = TokenValue::Str(":".to_string());
                    return Some(Token::new(TokenKind::Colon, symbol));
                }
                ';' => {
                    self.advance();
                    let symbol = TokenValue::Str(";".to_string());
                    return Some(Token::new(TokenKind::Semi, symbol));
                }
                '.' => {
                    self.advance();
                    let symbol = TokenValue::Str(".".to_string());
                    return Some(Token::new(TokenKind::Dot, symbol));
                }
                ',' => {
                    self.advance();
                    let symbol = TokenValue::Str(",".to_string());
                    return Some(Token::new(TokenKind::Comma, symbol));
                }
                _ => {
                    let symbol = TokenValue::Str("".to_string());
                    return Some(Token::new(TokenKind::EOF, symbol));
                }
            }
        }
        let symbol = TokenValue::Str("".to_string());
        Some(Token::new(TokenKind::EOF, symbol))
    }

    fn skip_comment(&mut self) {
        while self.current_char != '}' {
            self.advance();
        }
        self.advance();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("3 + 5".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_lexer2() {
        let mut lexer = Lexer::new("3 + 5 * 2".to_string());

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_lexer3() {
        let mut lexer = Lexer::new("3 + 5 * 2 - 1".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Minus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_lexer4() {
        let mut lexer = Lexer::new("3 + 5 * 2 - 1 / 2".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Minus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::FloatDivide);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_lexer5() {
        let mut lexer = Lexer::new("7 + 3 * (10 / (12 / (3 + 1) - 1))".to_string());

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::LParen);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::FloatDivide);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::LParen);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::FloatDivide);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::LParen);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::RParen);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Minus);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::RParen);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::RParen);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_with_program() {
        let mut lexer = Lexer::new("PROGRAM Part10; VAR Integer : INTEGER;".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Program);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Identifier);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Semi);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Var);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Identifier);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Colon);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Integer);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Semi);

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }
}
