#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Lambda,
    Symbol(String),
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::LeftBracket),
            ')' => tokens.push(Token::RightBracket),
            '\\' => tokens.push(Token::Lambda),
            c if c.is_ascii_whitespace() => (),
            _ => {
                let mut symbol = String::new();
                symbol.push(c);
                while let Some(&c) = chars.peek() {
                    if !c.is_ascii_whitespace() && c != '(' && c != ')' && c != '\\' {
                        symbol.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Symbol(symbol));
            }
        }
    }

    tokens
}
