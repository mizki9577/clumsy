use cst::{
    AbstractionExpression, ApplicationExpression, Expression, ExpressionStatement, Identifier,
    LetStatement, Number, Program, Statement, VariableExpression,
};
use lexer::Lexer;
use std::result;
use token::TokenType;

type Result<T> = result::Result<T, String>;

fn expect(lexer: &mut Lexer, expected: &TokenType) -> Result<()> {
    match lexer.next() {
        ref found if expected == found.get_type() => Ok(()),
        found => Err(format!("Expected {}, found {}", expected, found)),
    }
}

pub fn parse(lexer: &mut Lexer) -> Result<Program> {
    let mut statements = Vec::new();

    while let TokenType::Lambda
    | TokenType::LeftBracket
    | TokenType::Let
    | TokenType::Identifier(..)
    | TokenType::Number(..) = lexer.peek().get_type()
    {
        statements.push(parse_statement(lexer)?)
    }

    Ok(Program(statements))
}

fn parse_statement(lexer: &mut Lexer) -> Result<Statement> {
    let result = match lexer.peek().get_type() {
        TokenType::Lambda
        | TokenType::LeftBracket
        | TokenType::Identifier(..)
        | TokenType::Number(..) => {
            Statement::from(ExpressionStatement::new(parse_expression(lexer)?))
        }

        TokenType::Let => Statement::from(parse_let(lexer)?),

        ref found => {
            return Err(format!(
                "Expected '\\', '(', 'let' or identifier, found {}",
                found
            ))
        }
    };
    expect(lexer, &TokenType::Semicolon)?;
    Ok(result)
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Expression> {
    match lexer.peek().get_type() {
        TokenType::Lambda => Ok(Expression::from(parse_abstraction(lexer)?)),
        TokenType::LeftBracket | TokenType::Identifier(..) | TokenType::Number(..) => {
            Ok(Expression::from(parse_application(lexer)?))
        }
        ref found => Err(format!("Expected '\\', '(' or identifier, found {}", found)),
    }
}

fn parse_abstraction(lexer: &mut Lexer) -> Result<AbstractionExpression> {
    expect(lexer, &TokenType::Lambda)?;
    let parameters = parse_parameters(lexer)?;
    expect(lexer, &TokenType::Dot)?;
    let expression = parse_expression(lexer)?;
    Ok(AbstractionExpression::new(parameters, expression))
}

fn parse_parameters(lexer: &mut Lexer) -> Result<Vec<Identifier>> {
    let mut parameters = Vec::new();
    while let TokenType::Identifier(..) = lexer.peek().get_type() {
        parameters.push(parse_identifier(lexer)?);
    }
    Ok(parameters)
}

fn parse_application(lexer: &mut Lexer) -> Result<ApplicationExpression> {
    let mut expressions = Vec::new();
    loop {
        expressions.push(match lexer.peek().get_type() {
            TokenType::Identifier(..) => {
                Expression::from(VariableExpression::new(parse_identifier(lexer)?))
            }

            TokenType::Number(..) => Expression::from(parse_number(lexer)?),

            TokenType::LeftBracket => {
                expect(lexer, &TokenType::LeftBracket)?;
                let expression = parse_expression(lexer)?;
                expect(lexer, &TokenType::RightBracket)?;
                expression
            }

            TokenType::Lambda => Expression::from(parse_abstraction(lexer)?),

            _ => break,
        });
    }
    Ok(ApplicationExpression::new(expressions))
}

fn parse_identifier(lexer: &mut Lexer) -> Result<Identifier> {
    match lexer.next().get_type() {
        TokenType::Identifier(identifier) => Ok(Identifier::new(identifier.as_str())),
        found => Err(format!("Expected identifier, found {}", found)),
    }
}

fn parse_let(lexer: &mut Lexer) -> Result<LetStatement> {
    expect(lexer, &TokenType::Let)?;
    let variable = parse_identifier(lexer)?;
    expect(lexer, &TokenType::Equal)?;
    let expression = parse_expression(lexer)?;
    Ok(LetStatement::new(variable, expression))
}

fn parse_number(lexer: &mut Lexer) -> Result<Number> {
    match lexer.next().get_type() {
        TokenType::Number(number) => Ok(Number::new(number.as_str())),
        found => Err(format!("Expected number, found {}", found)),
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
