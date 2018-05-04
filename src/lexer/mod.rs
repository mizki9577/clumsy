#[cfg(test)]
mod tests;
mod token;

pub use self::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.chars().peekable(),
            line: 0,
            column: 0,
        }
    }

    fn source_next(&mut self) -> Option<char> {
        if let Some('\n') = self.source.peek() {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        self.source.next()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.source.peek() {
            if !c.is_ascii_whitespace() {
                break;
            }
            self.source_next();
        }

        let token_type = match self.source_next()? {
            '(' => TokenType::LeftBracket,
            ')' => TokenType::RightBracket,
            '\\' => TokenType::Lambda,
            '.' => TokenType::Dot,
            c if c.is_ascii_alphanumeric() || c == '-' || c == '_' => {
                let mut word = String::new();
                word.push(c);
                while let Some(&c) = self.source.peek() {
                    if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
                        break;
                    }
                    word.push(c);
                    self.source_next();
                }
                TokenType::Variable(word)
            }
            c => TokenType::InvalidCharacter(c),
        };

        Some(Token::new(token_type, self.line, self.column - 1))
    }
}
