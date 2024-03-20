use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Program,
    Begin,
    End,
    Plus,
    Minus,
    Multiply,
    IntegerDivide,
    EOF,
    LParen,
    RParen,
    Dot,
    Identifier,
    Assign,
    Semi,
    Var,
    Colon,
    Comma,
    Real,
    FloatDivide,
    Integer,
}

pub const RESERVED_KEYWORDS: [(TokenKind, &str); 7] = [
    (TokenKind::Begin, "BEGIN"),
    (TokenKind::End, "END"),
    (TokenKind::Program, "PROGRAM"),
    (TokenKind::Var, "VAR"),
    (TokenKind::Real, "REAL"),
    (TokenKind::Integer, "INTEGER"),
    (TokenKind::IntegerDivide, "DIV"),
];

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Int(i32),
    Real(f64),
    Str(String),
}

impl From<TokenValue> for i32 {
    fn from(value: TokenValue) -> i32 {
        match value {
            TokenValue::Int(i) => i,
            _ => panic!("Invalid token value"),
        }
    }
}

impl From<TokenValue> for f64 {
    fn from(value: TokenValue) -> f64 {
        match value {
            TokenValue::Real(r) => r,
            _ => panic!("Invalid token value"),
        }
    }
}

impl From<TokenValue> for String {
    fn from(value: TokenValue) -> String {
        match value {
            TokenValue::Str(s) => s,
            _ => panic!("Invalid token value"),
        }
    }
}

impl TokenValue {
    pub fn parse<T>(&self) -> T
    where
        T: From<Self>,
    {
        T::from(self.clone())
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: TokenValue) -> Token {
        Token { kind, value }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({:?}, {:?})", self.kind, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token() {
        let token = Token::new(TokenKind::Integer, TokenValue::Str("3".to_string()));
        assert_eq!(token.kind, TokenKind::Integer);
        match token.value {
            TokenValue::Str(s) => assert_eq!(s, "3"),
            _ => panic!("Invalid token value"),
        }
    }
}
