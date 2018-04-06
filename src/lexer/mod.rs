#[cfg(test)]
mod tests;

use std::iter::Peekable;
use std::result;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Lambda,
    Dot,
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub struct InvalidCharacter(char);

pub type Result = result::Result<Token, InvalidCharacter>;

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result;

    fn next(&mut self) -> Option<Result> {
        let c = self.source.next();
        match c {
            None => None,
            Some('(') => Some(Ok(Token::LeftBracket)),
            Some(')') => Some(Ok(Token::RightBracket)),
            Some('\\') => Some(Ok(Token::Lambda)),
            Some('.') => Some(Ok(Token::Dot)),
            Some(c) if c.is_ascii_whitespace() => self.next(),
            Some(c) if c.is_ascii_alphanumeric() || c == '-' || c == '_' => {
                let mut word = String::new();
                word.push(c);
                while let Some(&c) = self.source.peek() {
                    if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
                        break;
                    }
                    word.push(c);
                    self.source.next();
                }
                Some(Ok(Token::Variable(word)))
            }
            Some(c) => Some(Err(InvalidCharacter(c))),
        }
    }
}
