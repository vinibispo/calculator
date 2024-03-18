use crate::token::{Token, TokenKind};

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

    fn integer(&mut self) -> String {
        let mut result = String::new();
        while self.current_char != '\0' && self.current_char.is_numeric() {
            result.push(self.current_char);
            self.advance();
        }
        result
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while self.current_char != '\0' {
            if self.current_char.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if self.current_char.is_numeric() {
                return Some(Token::new(TokenKind::Number, self.integer()));
            }

            match self.current_char {
                '+' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Plus, "+".to_string()));
                }
                '-' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Minus, "-".to_string()));
                }
                '*' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Multiply, "*".to_string()));
                }
                '/' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Divide, "/".to_string()));
                }
                '(' => {
                    self.advance();
                    return Some(Token::new(TokenKind::LParen, "(".to_string()));
                }
                ')' => {
                    self.advance();
                    return Some(Token::new(TokenKind::RParen, ")".to_string()));
                }
                _ => {
                    return Some(Token::new(TokenKind::EOF, "".to_string()));
                }
            }
        }
        Some(Token::new(TokenKind::EOF, "".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("3 + 5".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "3".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "5".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
        assert_eq!(token.value, "".to_string());
    }

    #[test]
    fn test_lexer2() {
        let mut lexer = Lexer::new("3 + 5 * 2".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "3".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "5".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);
        assert_eq!(token.value, "*".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "2".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
        assert_eq!(token.value, "".to_string());
    }

    #[test]
    fn test_lexer3() {
        let mut lexer = Lexer::new("3 + 5 * 2 - 1".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "3".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "5".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);
        assert_eq!(token.value, "*".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "2".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Minus);
        assert_eq!(token.value, "-".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "1".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
        assert_eq!(token.value, "".to_string());
    }

    #[test]
    fn test_lexer4() {
        let mut lexer = Lexer::new("3 + 5 * 2 - 1 / 2".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "3".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "5".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);
        assert_eq!(token.value, "*".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "2".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Minus);
        assert_eq!(token.value, "-".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "1".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Divide);
        assert_eq!(token.value, "/".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "2".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
        assert_eq!(token.value, "".to_string());
    }

    #[test]
    fn test_lexer5() {
        let mut lexer = Lexer::new("7 + 3 * (10 / (12 / (3 + 1) - 1))".to_string());

        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "7".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "3".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Multiply);
        assert_eq!(token.value, "*".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::LParen);
        assert_eq!(token.value, "(".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "10".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Divide);
        assert_eq!(token.value, "/".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::LParen);
        assert_eq!(token.value, "(".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "12".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Divide);
        assert_eq!(token.value, "/".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::LParen);
        assert_eq!(token.value, "(".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "3".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.value, "+".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "1".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::RParen);
        assert_eq!(token.value, ")".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Minus);
        assert_eq!(token.value, "-".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "1".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::RParen);
        assert_eq!(token.value, ")".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::RParen);
        assert_eq!(token.value, ")".to_string());
        let token = lexer.get_next_token().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
        assert_eq!(token.value, "".to_string());
    }
}
