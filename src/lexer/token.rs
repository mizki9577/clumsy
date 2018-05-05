use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftBracket,
    RightBracket,
    Lambda,
    Dot,
    Variable(String),
    EOF,
    InvalidCharacter(char),
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Token {
        Token {
            token_type,
            line,
            column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} ({}:{})", self.token_type, self.line, self.column)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TokenType::LeftBracket => write!(f, "'('"),
            TokenType::RightBracket => write!(f, "')'"),
            TokenType::Lambda => write!(f, r"'\'"),
            TokenType::Dot => write!(f, "'.'"),
            TokenType::Variable(token) => write!(f, r#""{}""#, token.as_str()),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::InvalidCharacter(c) => write!(f, "'{}'", c),
        }
    }
}
