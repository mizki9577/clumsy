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
    let parameters = parse_parameters(tokens)?;
    expect(tokens, TokenType::Dot)?;
    let expression = parse_expression(tokens)?;
    Ok(ast::Expression::new_abstraction(parameters, expression))
}

fn parse_parameters(tokens: &mut Peekable<Lexer>) -> Result<Vec<ast::Variable>> {
    let mut parameters = vec![parse_variable(tokens)?];
    while let TokenType::Variable(_) = tokens.peek().unwrap().token_type {
        parameters.push(parse_variable(tokens)?);
    }
    Ok(parameters)
}

fn parse_application(tokens: &mut Peekable<Lexer>) -> Result<ast::Expression> {
    let mut expressions = Vec::new();
    loop {
        if let Some(token) = tokens.peek() {
            expressions.push(match token.token_type {
                TokenType::Variable(_) => ast::Expression::Variable(parse_variable(tokens)?),
                TokenType::LeftBracket => {
                    expect(tokens, TokenType::LeftBracket)?;
                    let expression = parse_expression(tokens)?;
                    expect(tokens, TokenType::RightBracket)?;
                    expression
                }
                _ => break,
            });
        } else {
            break;
        }
    }
    Ok(ast::Expression::new_application(expressions))
}

fn parse_variable(tokens: &mut Peekable<Lexer>) -> Result<ast::Variable> {
    let token = tokens.next().unwrap();
    match token.token_type {
        TokenType::Variable(variable) => Ok(ast::Variable::from(variable)),
        _ => Err(format!("Expected Variable, found {}", token)),
    }
}
