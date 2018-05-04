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
