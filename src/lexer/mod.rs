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

        let token_type = match self.source_next() {
            Some('(') => TokenType::LeftBracket,
            Some(')') => TokenType::RightBracket,
            Some('\\') => TokenType::Lambda,
            Some('.') => TokenType::Dot,
            Some('=') => TokenType::Equal,
            Some(';') => TokenType::Semicolon,
            Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                let mut word = String::new();
                word.push(c);
                while let Some(&c) = self.source.peek() {
                    if !c.is_ascii_alphanumeric() && c != '_' {
                        break;
                    }
                    word.push(c);
                    self.source_next();
                }
                if word == "let" {
                    TokenType::Let
                } else {
                    TokenType::Identifier(word)
                }
            }
            Some(c) => TokenType::InvalidCharacter(c),
            None => TokenType::EOF,
        };

        Some(Token::new(token_type, self.line, self.column - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_test() {
        assert_eq!(
            vec![
                Token::new(TokenType::LeftBracket, 0, 0),
                Token::new(TokenType::Lambda, 0, 1),
                Token::new(TokenType::Identifier("foo".to_owned()), 0, 4),
                Token::new(TokenType::Identifier("barBaz_2000".to_owned()), 1, 10),
                Token::new(TokenType::Dot, 1, 11),
                Token::new(TokenType::RightBracket, 1, 12),
            ],
            Lexer::new("(\\foo\nbarBaz_2000.)")
                .take_while(|token| token.token_type != TokenType::EOF)
                .collect::<Vec<_>>(),
        );
    }
}
