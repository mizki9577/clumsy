use std::iter::Peekable;
use std::str::Chars;
use token::{Token, TokenKind};

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    state: LexerState,
}

enum LexerState {
    Initial,
    Return(Option<TokenKind>),
    Word(Option<String>),
    Whitespace,
    FirstSlash,
    Comment,
    Number(Option<String>),
    Character(Option<char>),
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Peekable<Lexer<'a>> {
        Lexer {
            source: source.chars().peekable(),
            line: 0,
            column: 0,
            state: LexerState::Initial,
        }.peekable()
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

    fn next(&mut self) -> Option<Token> {
        loop {
            self.state = match self.state {
                LexerState::Initial => match self.source_next() {
                    Some('(') => LexerState::Return(Some(TokenKind::LeftBracket)),
                    Some(')') => LexerState::Return(Some(TokenKind::RightBracket)),
                    Some('\\') => LexerState::Return(Some(TokenKind::Lambda)),
                    Some('.') => LexerState::Return(Some(TokenKind::Dot)),
                    Some('=') => LexerState::Return(Some(TokenKind::Equal)),
                    Some(';') => LexerState::Return(Some(TokenKind::Semicolon)),
                    Some('/') => LexerState::FirstSlash,
                    Some(c) if c.is_ascii_whitespace() => LexerState::Whitespace,
                    Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                        LexerState::Word(Some(c.to_string()))
                    }
                    Some('\'') => LexerState::Character(None),
                    Some(c) if c.is_ascii_digit() => LexerState::Number(Some(c.to_string())),
                    Some(c) => LexerState::Return(Some(TokenKind::InvalidCharacter(c))),
                    None => LexerState::Return(None),
                },

                LexerState::Return(ref mut kind) => {
                    let kind = kind.take();
                    self.state = LexerState::Initial;
                    return Some(Token::new(kind, self.line, self.column - 1));
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
                            TokenKind::Let
                        } else {
                            TokenKind::Identifier(word)
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
                    _ => LexerState::Return(Some(TokenKind::InvalidCharacter('/'))),
                },

                LexerState::Comment => match self.source_next() {
                    Some('\n') => LexerState::Initial,
                    _ => LexerState::Comment,
                },

                LexerState::Number(ref mut number) => {
                    let mut number = number.take().unwrap();
                    match self.source.peek() {
                        Some(&c) if c.is_ascii_digit() => {
                            number.push(c);
                            self.source_next();
                            LexerState::Number(Some(number))
                        }

                        _ => LexerState::Return(Some(TokenKind::Number(number))),
                    }
                }

                LexerState::Character(None) => match self.source_next() {
                    Some('\'') => LexerState::Return(Some(TokenKind::InvalidCharacter('\''))),
                    Some(character) => LexerState::Character(Some(character)),
                    None => LexerState::Return(None),
                },

                LexerState::Character(Some(character)) => match self.source_next() {
                    Some('\'') => LexerState::Return(Some(TokenKind::Character(character))),
                    Some(character) => {
                        LexerState::Return(Some(TokenKind::InvalidCharacter(character)))
                    }
                    None => LexerState::Return(None),
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_test() {
        let lexer = Lexer::new("(\\foo\nbarBaz_2000'*'//@@@@\n.)42^");
        let results = vec![
            Token::new(TokenKind::LeftBracket, 0, 0),
            Token::new(TokenKind::Lambda, 0, 1),
            Token::new(TokenKind::Identifier("foo".to_owned()), 0, 4),
            Token::new(TokenKind::Identifier("barBaz_2000".to_owned()), 1, 10),
            Token::new(TokenKind::Character('*'), 1, 13),
            Token::new(TokenKind::Dot, 2, 0),
            Token::new(TokenKind::RightBracket, 2, 1),
            Token::new(TokenKind::Number("42".to_owned()), 2, 3),
            Token::new(TokenKind::InvalidCharacter('^'), 2, 4),
        ].into_iter();

        for (expected, result) in lexer.zip(results) {
            assert_eq!(expected, result);
        }
    }
}
