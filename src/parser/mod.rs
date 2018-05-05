pub mod ast;
#[cfg(test)]
mod tests;

use lexer::{Lexer, TokenType};
use std::iter::Peekable;
use std::result;

type Result<T> = result::Result<T, String>;

fn expect(tokens: &mut Peekable<Lexer>, expected: TokenType) -> Result<()> {
    match tokens.next().unwrap() {
        ref found if expected == found.token_type => Ok(()),
        found => Err(format!("Expected {}, found {}", expected, found)),
    }
}

pub fn parse_expression(tokens: &mut Peekable<Lexer>) -> Result<ast::Expression> {
    let token = tokens.peek().unwrap();
    match token.token_type {
        TokenType::Lambda => parse_abstraction(tokens),
        TokenType::LeftBracket | TokenType::Variable(_) => parse_application(tokens),
        _ => Err(format!("Expected '\\', '(' or Variable, found {}", token)),
    }
}

fn parse_abstraction(tokens: &mut Peekable<Lexer>) -> Result<ast::Expression> {
    expect(tokens, TokenType::Lambda)?;
    parse_abstraction_body(tokens)
}

fn parse_abstraction_body(tokens: &mut Peekable<Lexer>) -> Result<ast::Expression> {
    let parameter = parse_variable(tokens)?;
    let token = tokens.peek().unwrap();
    let expression = box match token.token_type {
        TokenType::Variable(_) => parse_abstraction_body(tokens)?,
        TokenType::Dot => {
            expect(tokens, TokenType::Dot)?;
            parse_expression(tokens)?
        }
        _ => return Err(format!("Expected '.' or Variable, found {}", token)),
    };
    Ok(ast::Expression::Abstraction {
        parameter,
        expression,
    })
}

fn parse_application(tokens: &mut Peekable<Lexer>) -> Result<ast::Expression> {
    let mut items = Vec::new();
    loop {
        if let Some(token) = tokens.peek() {
            items.push(match token.token_type {
                TokenType::Variable(_) => parse_variable(tokens).map(ast::Expression::Variable)?,
                TokenType::LeftBracket => {
                    expect(tokens, TokenType::LeftBracket)?;
                    let result = parse_expression(tokens)?;
                    expect(tokens, TokenType::RightBracket)?;
                    result
                }
                _ => break,
            });
        } else {
            break;
        }
    }
    Ok(fix_application(items))
}

fn fix_application(mut items: Vec<ast::Expression>) -> ast::Expression {
    let last = items.pop().expect("Application list is empty!");
    if items.is_empty() {
        last
    } else {
        ast::Expression::Application {
            callee: box fix_application(items),
            argument: box last,
        }
    }
}

fn parse_variable(tokens: &mut Peekable<Lexer>) -> Result<ast::Variable> {
    let token = tokens.next().unwrap();
    match token.token_type {
        TokenType::Variable(ref variable) => Ok(ast::Variable::new(variable)),
        _ => Err(format!("Expected Variable, found {}", token)),
    }
}
