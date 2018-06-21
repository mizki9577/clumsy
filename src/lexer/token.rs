use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    column: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftBracket,
    RightBracket,
    Lambda,
    Dot,
    Equal,
    Semicolon,
    Let,
    Identifier(String),
    Number(String),
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

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
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
            TokenType::Equal => write!(f, "'='"),
            TokenType::Semicolon => write!(f, "';'"),
            TokenType::Let => write!(f, "'let'"),
            TokenType::Identifier(identifier) => write!(f, r#""{}""#, identifier),
            TokenType::Number(number) => write!(f, r#""{}""#, number),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::InvalidCharacter(c) => write!(f, "'{}'", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_display() {
        for (token, result) in &[
            (Token::new(TokenType::LeftBracket, 0, 0), "'('"),
            (Token::new(TokenType::RightBracket, 0, 0), "')'"),
            (Token::new(TokenType::Lambda, 0, 0), r"'\'"),
            (Token::new(TokenType::Dot, 0, 0), "'.'"),
            (Token::new(TokenType::Equal, 0, 0), "'='"),
            (Token::new(TokenType::Semicolon, 0, 0), "';'"),
            (Token::new(TokenType::Let, 0, 0), "'let'"),
            (
                Token::new(TokenType::Identifier("x".to_owned()), 0, 0),
                r#""x""#,
            ),
            (Token::new(TokenType::EOF, 0, 0), "EOF"),
            (Token::new(TokenType::InvalidCharacter('?'), 0, 0), "'?'"),
        ] {
            assert_eq!(format!("{}", token), format!("{} (0:0)", result));
        }
    }
}
