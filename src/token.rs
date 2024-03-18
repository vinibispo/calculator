use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    EOF,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Token { kind, value }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({:?}, {})", self.kind, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token() {
        let token = Token::new(TokenKind::Number, "3".to_string());
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "3".to_string());
    }
}
