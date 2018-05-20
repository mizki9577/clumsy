pub mod ast;

use self::ast::*;
use lexer::{Lexer, TokenType};
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

pub fn parse(tokens: &mut Peekable<Lexer>) -> Result<AST> {
    match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Lambda | TokenType::LeftBracket | TokenType::Identifier(_) => {
                Ok(AST::Expression(parse_expression(tokens)?))
            }
            TokenType::Let => Ok(AST::Let(parse_let(tokens)?)),
            ref found => Err(format!(
                "Expected '\\', '(', 'let' or Variable, found {}",
                found
            )),
        },
        None => unreachable!(),
    }
}

pub fn parse_expression(tokens: &mut Peekable<Lexer>) -> Result<ASTExpression> {
    match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Lambda => Ok(ASTExpression::Abstraction(parse_abstraction(tokens)?)),
            TokenType::LeftBracket | TokenType::Identifier(_) => {
                Ok(ASTExpression::Application(parse_application(tokens)?))
            }
            ref found => Err(format!("Expected '\\', '(' or Variable, found {}", found)),
        },
        None => unreachable!(),
    }
}

fn parse_abstraction(tokens: &mut Peekable<Lexer>) -> Result<ASTAbstraction> {
    expect(tokens, &TokenType::Lambda)?;
    let parameters = parse_parameters(tokens)?;
    expect(tokens, &TokenType::Dot)?;
    let expression = parse_expression(tokens)?;
    Ok(ASTAbstraction::new(parameters, expression))
}

fn parse_parameters(tokens: &mut Peekable<Lexer>) -> Result<Vec<ASTIdentifier>> {
    let mut parameters = Vec::new();
    loop {
        if let Some(token) = tokens.peek() {
            if let TokenType::Identifier(_) = token.token_type {
                parameters.push(parse_identifier(tokens)?);
            } else {
                break;
            }
        } else {
            unreachable!()
        }
    }
    Ok(parameters)
}

fn parse_application(tokens: &mut Peekable<Lexer>) -> Result<ASTApplication> {
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
            TokenType::Lambda => ASTExpression::Abstraction(parse_abstraction(tokens)?),
            _ => break,
        });
    }
    Ok(ASTApplication::new(expressions))
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
    expect(tokens, &TokenType::Let)?;
    let variable = parse_identifier(tokens)?;
    expect(tokens, &TokenType::Equal)?;
    let expression = parse_expression(tokens)?;
    Ok(ASTLet::new(variable, expression))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_abstraction() {
        let result = parse_abstraction(&mut Lexer::new("\\x y. x").peekable());
        let expected = Ok(ASTAbstraction::new(
            vec![ASTIdentifier::from("x"), ASTIdentifier::from("y")],
            ASTExpression::Application(ASTApplication::new(vec![ASTExpression::Identifier(
                ASTIdentifier::from("x"),
            )])),
        ));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_application() {
        let result = parse_application(&mut Lexer::new("x y z").peekable());
        let expected = Ok(ASTApplication::new(vec![
            ASTExpression::Identifier(ASTIdentifier::from("x")),
            ASTExpression::Identifier(ASTIdentifier::from("y")),
            ASTExpression::Identifier(ASTIdentifier::from("z")),
        ]));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_paratemers() {
        let result = parse_parameters(&mut Lexer::new("x y z").peekable());
        let expected = Ok(vec![
            ASTIdentifier::from("x"),
            ASTIdentifier::from("y"),
            ASTIdentifier::from("z"),
        ]);
        assert_eq!(expected, result);
    }
}
