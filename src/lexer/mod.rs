mod token;

pub use self::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    state: LexerState,
}

enum LexerState {
    Initial,
    Return(Option<TokenType>),
    Word(Option<String>),
    Whitespace,
    FirstSlash,
    Comment,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.chars().peekable(),
            line: 0,
            column: 0,
            state: LexerState::Initial,
        }
    }

    fn next_token(&mut self) -> Token {
        loop {
            self.state = match self.state {
                LexerState::Initial => match self.source_next() {
                    Some('(') => LexerState::Return(Some(TokenType::LeftBracket)),
                    Some(')') => LexerState::Return(Some(TokenType::RightBracket)),
                    Some('\\') => LexerState::Return(Some(TokenType::Lambda)),
                    Some('.') => LexerState::Return(Some(TokenType::Dot)),
                    Some('=') => LexerState::Return(Some(TokenType::Equal)),
                    Some(';') => LexerState::Return(Some(TokenType::Semicolon)),
                    Some('/') => LexerState::FirstSlash,
                    Some(c) if c.is_ascii_whitespace() => LexerState::Whitespace,
                    Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                        LexerState::Word(Some(c.to_string()))
                    }
                    Some(c) => LexerState::Return(Some(TokenType::InvalidCharacter(c))),
                    None => LexerState::Return(Some(TokenType::EOF)),
                },

                LexerState::Return(ref mut token_type) => {
                    let token_type = token_type.take().unwrap();
                    self.state = LexerState::Initial;
                    return Token::new(token_type, self.line, self.column - 1);
                }

                LexerState::Word(ref mut word) => {
                    let mut word = word.take().unwrap();
                    match self.source.peek() {
                        Some(&c) if c.is_ascii_alphanumeric() || c == '_' => {
                            word.push(c);
                            self.source_next();
                            LexerState::Word(Some(word))
                        }

                        _ => LexerState::Return(Some(if word == "let" {
                            TokenType::Let
                        } else {
                            TokenType::Identifier(word)
                        })),
                    }
                }

                LexerState::Whitespace => match self.source.peek() {
                    Some(c) if c.is_ascii_whitespace() => {
                        self.source_next();
                        LexerState::Whitespace
                    }
                    _ => LexerState::Initial,
                },

                LexerState::FirstSlash => match self.source.peek() {
                    Some('/') => LexerState::Comment,
                    _ => LexerState::Return(Some(TokenType::InvalidCharacter('/'))),
                },

                LexerState::Comment => match self.source_next() {
                    Some('\n') => LexerState::Initial,
                    _ => LexerState::Comment,
                },
            }
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
        Some(self.next_token())
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
                Token::new(TokenType::Dot, 2, 0),
                Token::new(TokenType::RightBracket, 2, 1),
                Token::new(TokenType::InvalidCharacter('^'), 2, 2),
            ],
            Lexer::new("(\\foo\nbarBaz_2000//@@@@\n.)^")
                .take_while(|token| token.token_type != TokenType::EOF)
                .collect::<Vec<_>>(),
        );
    }
}
