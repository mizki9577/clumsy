#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Lambda,
    Dot,
    Variable(String),
    InvalidCharacter(char),
}
