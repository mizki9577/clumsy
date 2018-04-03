use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Lambda,
    Symbol(String),
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

    fn next(&mut self) -> Option<Token> {
        let c = self.source.next();
        match c {
            None => None,
            Some('(') => Some(Token::LeftBracket),
            Some(')') => Some(Token::RightBracket),
            Some('\\') => Some(Token::Lambda),
            Some(c) if c.is_ascii_whitespace() => self.next(),
            Some(c) => {
                let mut symbol = String::new();
                symbol.push(c);
                while let Some(&c) = self.source.peek() {
                    if c.is_ascii_whitespace() || c == '(' || c == ')' || c == '\\' {
                        break;
                    }
                    symbol.push(c);
                    self.source.next();
                }
                Some(Token::Symbol(symbol))
            }
        }
    }
}
