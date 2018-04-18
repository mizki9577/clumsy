#[cfg(test)]
mod tests;

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Lambda,
    Dot,
    Variable(String),
    InvalidCharacter(char),
}

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
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.source.peek() {
            if !c.is_ascii_whitespace() {
                break;
            }
            self.source.next();
        }

        let c = self.source.next();
        match c {
            None => None,
            Some('(') => Some(Token::LeftBracket),
            Some(')') => Some(Token::RightBracket),
            Some('\\') => Some(Token::Lambda),
            Some('.') => Some(Token::Dot),
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
                Some(Token::Variable(word))
            }
            Some(c) => Some(Token::InvalidCharacter(c)),
        }
    }
}
