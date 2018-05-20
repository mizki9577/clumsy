#[cfg(test)]
mod tests;

pub mod ast;

use self::ast::*;
use lexer::{Lexer, TokenType};
use std::iter::Peekable;
use std::result;

type Result<T> = result::Result<T, String>;

fn expect(tokens: &mut Peekable<Lexer>, expected: &TokenType) -> Result<()> {
    match tokens.next().unwrap() {
        ref found if expected == &found.token_type => Ok(()),
        found => Err(format!("Expected {}, found {}", expected, found)),
    }
}

pub fn parse_expression(tokens: &mut Peekable<Lexer>) -> Result<ASTExpression> {
    let token = tokens.peek().unwrap();
    match token.token_type {
        TokenType::Lambda => parse_abstraction(tokens),
        TokenType::LeftBracket | TokenType::Identifier(_) => parse_application(tokens),
        _ => Err(format!("Expected '\\', '(' or Variable, found {}", token)),
    }
}

fn parse_abstraction(tokens: &mut Peekable<Lexer>) -> Result<ASTExpression> {
    expect(tokens, &TokenType::Lambda)?;
    let parameters = parse_parameters(tokens)?;
    expect(tokens, &TokenType::Dot)?;
    let expression = parse_expression(tokens)?;
    Ok(ASTExpression::Abstraction(ASTAbstraction::new(
        parameters, expression,
    )))
}

fn parse_parameters(tokens: &mut Peekable<Lexer>) -> Result<Vec<ASTIdentifier>> {
    let mut parameters = vec![parse_identifier(tokens)?];
    while let TokenType::Identifier(_) = tokens.peek().unwrap().token_type {
        parameters.push(parse_identifier(tokens)?);
    }
    Ok(parameters)
}

fn parse_application(tokens: &mut Peekable<Lexer>) -> Result<ASTExpression> {
    let mut expressions = Vec::new();
    while let Some(token) = tokens.peek() {
        expressions.push(match token.token_type {
            TokenType::Identifier(_) => ASTExpression::Identifier(parse_identifier(tokens)?),
            TokenType::LeftBracket => {
                expect(tokens, &TokenType::LeftBracket)?;
                let expression = parse_expression(tokens)?;
                expect(tokens, &TokenType::RightBracket)?;
                expression
            }
            TokenType::Lambda => parse_abstraction(tokens)?,
            _ => break,
        });
    }
    Ok(ASTExpression::Application(ASTApplication::new(expressions)))
}

fn parse_identifier(tokens: &mut Peekable<Lexer>) -> Result<ASTIdentifier> {
    let token = tokens.next().unwrap();
    match token.token_type {
        TokenType::Identifier(variable) => Ok(ASTIdentifier::from(variable)),
        _ => Err(format!("Expected Variable, found {}", token)),
    }
}

fn parse_let(tokens: &mut Peekable<Lexer>) -> Result<ASTLet> {
    unimplemented!()
}
