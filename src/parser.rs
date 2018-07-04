use cst::{
    AbstractionExpression, ApplicationExpression, Expression, ExpressionStatement, Identifier,
    LetStatement, Number, Program, Statement, VariableExpression,
};
use lexer::Lexer;
use std::iter::Peekable;
use std::result;
use token::{Token, TokenKind};

type Result<T> = result::Result<T, String>;

static UNEXPECTED_NONE: &str = "Unexpected None";

fn expect(lexer: &mut Peekable<Lexer>, expected: &TokenKind) -> Result<()> {
    match lexer.next().unwrap_or_else(|| panic!(UNEXPECTED_NONE)) {
        Token {
            kind: Some(ref found),
            ..
        } if expected == found =>
        {
            Ok(())
        }
        found => Err(format!("Expected {}, found {}", expected, found)),
    }
}

pub fn parse(lexer: &mut Peekable<Lexer>) -> Result<Program> {
    let mut statements = Vec::new();

    while let Some(TokenKind::Lambda)
    | Some(TokenKind::LeftBracket)
    | Some(TokenKind::Let)
    | Some(TokenKind::Identifier(..))
    | Some(TokenKind::Number(..)) =
        lexer.peek().unwrap_or_else(|| panic!(UNEXPECTED_NONE)).kind
    {
        statements.push(parse_statement(lexer)?)
    }

    Ok(Program(statements))
}

fn parse_statement(lexer: &mut Peekable<Lexer>) -> Result<Statement> {
    let token = lexer.peek().unwrap_or_else(|| panic!(UNEXPECTED_NONE));

    let result = match token.kind {
        Some(TokenKind::Lambda)
        | Some(TokenKind::LeftBracket)
        | Some(TokenKind::Identifier(..))
        | Some(TokenKind::Number(..)) => {
            Statement::from(ExpressionStatement::new(parse_expression(lexer)?))
        }

        Some(TokenKind::Let) => Statement::from(parse_let(lexer)?),

        _ => {
            return Err(format!(
                "Expected '\\', '(', 'let' or identifier, found {}",
                token
            ))
        }
    };
    expect(lexer, &TokenKind::Semicolon)?;
    Ok(result)
}

pub fn parse_expression(lexer: &mut Peekable<Lexer>) -> Result<Expression> {
    let token = lexer.peek().unwrap_or_else(|| panic!(UNEXPECTED_NONE));

    match token.kind {
        Some(TokenKind::Lambda) => Ok(Expression::from(parse_abstraction(lexer)?)),
        Some(TokenKind::LeftBracket)
        | Some(TokenKind::Identifier(..))
        | Some(TokenKind::Number(..)) => Ok(Expression::from(parse_application(lexer)?)),
        _ => Err(format!("Expected '\\', '(' or identifier, found {}", token)),
    }
}

fn parse_abstraction(lexer: &mut Peekable<Lexer>) -> Result<AbstractionExpression> {
    expect(lexer, &TokenKind::Lambda)?;
    let parameters = parse_parameters(lexer)?;
    expect(lexer, &TokenKind::Dot)?;
    let expression = parse_expression(lexer)?;
    Ok(AbstractionExpression::new(parameters, expression))
}

fn parse_parameters(lexer: &mut Peekable<Lexer>) -> Result<Vec<Identifier>> {
    let mut parameters = Vec::new();
    while let Some(TokenKind::Identifier(..)) =
        lexer.peek().unwrap_or_else(|| panic!(UNEXPECTED_NONE)).kind
    {
        parameters.push(parse_identifier(lexer)?);
    }
    Ok(parameters)
}

fn parse_application(lexer: &mut Peekable<Lexer>) -> Result<ApplicationExpression> {
    let mut expressions = Vec::new();
    loop {
        expressions.push(
            match lexer.peek().unwrap_or_else(|| panic!(UNEXPECTED_NONE)).kind {
                Some(TokenKind::Identifier(..)) => {
                    Expression::from(VariableExpression::new(parse_identifier(lexer)?))
                }

                Some(TokenKind::Number(..)) => Expression::from(parse_number(lexer)?),

                Some(TokenKind::LeftBracket) => {
                    expect(lexer, &TokenKind::LeftBracket)?;
                    let expression = parse_expression(lexer)?;
                    expect(lexer, &TokenKind::RightBracket)?;
                    expression
                }

                Some(TokenKind::Lambda) => Expression::from(parse_abstraction(lexer)?),

                _ => break,
            },
        );
    }
    Ok(ApplicationExpression::new(expressions))
}

fn parse_identifier(lexer: &mut Peekable<Lexer>) -> Result<Identifier> {
    let token = lexer.next().unwrap_or_else(|| panic!(UNEXPECTED_NONE));

    match token.kind {
        Some(TokenKind::Identifier(identifier)) => Ok(Identifier::new(identifier.as_str())),
        _ => Err(format!("Expected identifier, found {}", token)),
    }
}

fn parse_let(lexer: &mut Peekable<Lexer>) -> Result<LetStatement> {
    expect(lexer, &TokenKind::Let)?;
    let variable = parse_identifier(lexer)?;
    expect(lexer, &TokenKind::Equal)?;
    let expression = parse_expression(lexer)?;
    Ok(LetStatement::new(variable, expression))
}

fn parse_number(lexer: &mut Peekable<Lexer>) -> Result<Number> {
    let token = lexer.next().unwrap_or_else(|| panic!(UNEXPECTED_NONE));

    match token.kind {
        Some(TokenKind::Number(number)) => Ok(Number::new(number.as_str())),
        _ => Err(format!("Expected number, found {}", token)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_abstraction() {
        let result = parse_abstraction(&mut Lexer::new("\\x y. x"));
        let expected = Ok(AbstractionExpression::new(
            vec![Identifier::new("x"), Identifier::new("y")],
            Expression::from(ApplicationExpression::new(vec![Expression::from(
                VariableExpression::new(Identifier::new("x")),
            )])),
        ));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_application() {
        let result = parse_application(&mut Lexer::new("x y z"));
        let expected = Ok(ApplicationExpression::new(vec![
            Expression::from(VariableExpression::new(Identifier::new("x"))),
            Expression::from(VariableExpression::new(Identifier::new("y"))),
            Expression::from(VariableExpression::new(Identifier::new("z"))),
        ]));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_paratemers() {
        let result = parse_parameters(&mut Lexer::new("x y z"));
        let expected = Ok(vec![
            Identifier::new("x"),
            Identifier::new("y"),
            Identifier::new("z"),
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_let() {
        let result = parse_let(&mut Lexer::new("let x = y"));
        let expected = Ok(LetStatement::new(
            Identifier::new("x"),
            ApplicationExpression::new(vec![Expression::from(VariableExpression::new(
                Identifier::new("y"),
            ))]),
        ));
        assert_eq!(expected, result);
    }
}
