use ast;
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

pub fn parse(tokens: &mut Peekable<Lexer>) -> Result<ast::Program> {
    let mut statements = Vec::new();

    while let Some(token) = tokens.peek() {
        match token.token_type {
            TokenType::Lambda
            | TokenType::LeftBracket
            | TokenType::Let
            | TokenType::Identifier(_) => statements.push(parse_statement(tokens)?),
            _ => break,
        }
    }

    Ok(ast::Program(statements))
}

fn parse_statement(tokens: &mut Peekable<Lexer>) -> Result<ast::Statement> {
    let result = match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Lambda | TokenType::LeftBracket | TokenType::Identifier(_) => {
                ast::Statement::from(parse_expression(tokens)?)
            }

            TokenType::Let => ast::Statement::from(parse_let(tokens)?),

            ref found => {
                return Err(format!(
                    "Expected '\\', '(', 'let' or Variable, found {}",
                    found
                ))
            }
        },
        None => unreachable!(),
    };
    expect(tokens, &TokenType::Semicolon)?;
    Ok(result)
}

pub fn parse_expression(tokens: &mut Peekable<Lexer>) -> Result<ast::Expression> {
    match tokens.peek() {
        Some(token) => match token.token_type {
            TokenType::Lambda => Ok(ast::Expression::from(parse_abstraction(tokens)?)),
            TokenType::LeftBracket | TokenType::Identifier(_) => {
                Ok(ast::Expression::from(parse_application(tokens)?))
            }
            ref found => Err(format!("Expected '\\', '(' or Variable, found {}", found)),
        },
        None => unreachable!(),
    }
}

fn parse_abstraction(tokens: &mut Peekable<Lexer>) -> Result<ast::AbstractionExpression> {
    expect(tokens, &TokenType::Lambda)?;
    let parameters = parse_parameters(tokens)?;
    expect(tokens, &TokenType::Dot)?;
    let expression = parse_expression(tokens)?;
    Ok(ast::AbstractionExpression::new(parameters, expression))
}

fn parse_parameters(tokens: &mut Peekable<Lexer>) -> Result<Vec<ast::Identifier>> {
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

fn parse_application(tokens: &mut Peekable<Lexer>) -> Result<ast::ApplicationExpression> {
    let mut expressions = Vec::new();
    while let Some(token) = tokens.peek() {
        expressions.push(match token.token_type {
            TokenType::Identifier(_) => ast::Expression::from(parse_identifier(tokens)?),
            TokenType::LeftBracket => {
                expect(tokens, &TokenType::LeftBracket)?;
                let expression = parse_expression(tokens)?;
                expect(tokens, &TokenType::RightBracket)?;
                expression
            }
            TokenType::Lambda => ast::Expression::from(parse_abstraction(tokens)?),
            _ => break,
        });
    }
    Ok(ast::ApplicationExpression::new(expressions))
}

fn parse_identifier(tokens: &mut Peekable<Lexer>) -> Result<ast::Identifier> {
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::Identifier(variable) => Ok(ast::Identifier::new(variable)),
            found => Err(format!("Expected Variable, found {}", found)),
        },
        None => unreachable!(),
    }
}

fn parse_let(tokens: &mut Peekable<Lexer>) -> Result<ast::LetStatement> {
    expect(tokens, &TokenType::Let)?;
    let variable = parse_identifier(tokens)?;
    expect(tokens, &TokenType::Equal)?;
    let expression = parse_expression(tokens)?;
    Ok(ast::LetStatement::new(variable, expression))
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
