use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: Option<TokenKind>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftBracket,
    RightBracket,
    Lambda,
    Dot,
    Equal,
    Semicolon,
    Let,
    Identifier(String),
    Number(String),
    Character(char),
    InvalidCharacter(char),
}

impl Token {
    pub fn new<T>(kind: T, line: usize, column: usize) -> Token
    where
        T: Into<Option<TokenKind>>,
    {
        Token {
            kind: kind.into(),
            line,
            column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(ref kind) = self.kind {
            write!(f, "{} ({}:{})", kind, self.line, self.column)
        } else {
            write!(f, "None ({}:{})", self.line, self.column)
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TokenKind::LeftBracket => write!(f, "'('"),
            TokenKind::RightBracket => write!(f, "')'"),
            TokenKind::Lambda => write!(f, r"'\'"),
            TokenKind::Dot => write!(f, "'.'"),
            TokenKind::Equal => write!(f, "'='"),
            TokenKind::Semicolon => write!(f, "';'"),
            TokenKind::Let => write!(f, "'let'"),
            TokenKind::Identifier(identifier) => write!(f, r#""{}""#, identifier),
            TokenKind::Number(number) => write!(f, r#""{}""#, number),
            TokenKind::Character(character) => write!(f, "'{}'", character),
            TokenKind::InvalidCharacter(c) => write!(f, "'{}'", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_display() {
        for (token, result) in &[
            (Token::new(TokenKind::LeftBracket, 0, 0), "'('"),
            (Token::new(TokenKind::RightBracket, 0, 0), "')'"),
            (Token::new(TokenKind::Lambda, 0, 0), r"'\'"),
            (Token::new(TokenKind::Dot, 0, 0), "'.'"),
            (Token::new(TokenKind::Equal, 0, 0), "'='"),
            (Token::new(TokenKind::Semicolon, 0, 0), "';'"),
            (Token::new(TokenKind::Let, 0, 0), "'let'"),
            (
                Token::new(TokenKind::Identifier("x".to_owned()), 0, 0),
                r#""x""#,
            ),
            (Token::new(TokenKind::InvalidCharacter('?'), 0, 0), "'?'"),
        ] {
            assert_eq!(format!("{}", token), format!("{} (0:0)", result));
        }
    }
}
