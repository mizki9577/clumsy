pub mod ast;

use self::ast::*;
use lexer::{Lexer, Token, TokenType};
use std::iter::Peekable;
use std::result;

type Result<T> = result::Result<T, String>;

fn expect(tokens: &mut Peekable<Lexer>, expected: &TokenType) -> Result<()> {
    match tokens.next() {
        Some(ref found) if expected == &found.token_type => Ok(()),
        Some(found) => Err(format!("Expected {}, found {}", expected, found)),
        None => unreachable!(),
    }
}

pub fn parse_expression(tokens: &mut Peekable<Lexer>) -> Result<ASTExpression> {
    match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Lambda => parse_abstraction(tokens),
            TokenType::LeftBracket | TokenType::Identifier(_) => parse_application(tokens),
            ref found => Err(format!("Expected '\\', '(' or Variable, found {}", found)),
        },
        None => unreachable!(),
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
    while let Some(Token {
        token_type: TokenType::Identifier(_),
        ..
    }) = tokens.peek()
    {
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
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::Identifier(variable) => Ok(ASTIdentifier::from(variable)),
            found => Err(format!("Expected Variable, found {}", found)),
        },
        None => unreachable!(),
    }
}

fn parse_let(tokens: &mut Peekable<Lexer>) -> Result<ASTLet> {
    unimplemented!()
}

#[test]
fn test_parse_abstraction() {
    let result = parse_abstraction(&mut Lexer::new("\\x y. x").peekable());
    let expected = Ok(ASTExpression::Abstraction(ASTAbstraction::new(
        vec![ASTIdentifier::from("x"), ASTIdentifier::from("y")],
        ASTExpression::Application(ASTApplication::new(vec![ASTExpression::Identifier(
            ASTIdentifier::from("x"),
        )])),
    )));
    assert_eq!(expected, result);
}

#[test]
fn test_parse_application() {
    let result = parse_application(&mut Lexer::new("x y z").peekable());
    let expected = Ok(ASTExpression::Application(ASTApplication::new(vec![
        ASTExpression::Identifier(ASTIdentifier::from("x")),
        ASTExpression::Identifier(ASTIdentifier::from("y")),
        ASTExpression::Identifier(ASTIdentifier::from("z")),
    ])));
    assert_eq!(expected, result);
}
