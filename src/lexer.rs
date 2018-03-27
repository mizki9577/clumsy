#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Lambda,
    Symbol(String),
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut symbol = String::new();
    let chars = source.chars();

    for c in chars {
        if (c == '(' || c == ')' || c == '\\' || c.is_ascii_whitespace()) && !symbol.is_empty() {
            tokens.push(Token::Symbol(symbol.clone()));
            symbol.clear();
        }

        if c == '(' {
            tokens.push(Token::LeftBracket);
        } else if c == ')' {
            tokens.push(Token::RightBracket);
        } else if c == '\\' {
            tokens.push(Token::Lambda);
        } else if !c.is_ascii_whitespace() {
            symbol.push(c);
        }
    }
    if !symbol.is_empty() {
        tokens.push(Token::Symbol(symbol.clone()));
        symbol.clear();
    }

    tokens
}
