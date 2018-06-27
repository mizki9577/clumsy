use cst;
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

pub fn parse(lexer: &mut Lexer) -> Result<cst::Program> {
    let mut statements = Vec::new();

    while let TokenType::Lambda
    | TokenType::LeftBracket
    | TokenType::Let
    | TokenType::Identifier(..)
    | TokenType::Number(..) = lexer.peek().get_type()
    {
        statements.push(parse_statement(lexer)?)
    }

    Ok(cst::Program(statements))
}

fn parse_statement(lexer: &mut Lexer) -> Result<cst::Statement> {
    let result = match lexer.peek().get_type() {
        TokenType::Lambda
        | TokenType::LeftBracket
        | TokenType::Identifier(..)
        | TokenType::Number(..) => {
            cst::Statement::from(cst::ExpressionStatement::new(parse_expression(lexer)?))
        }

        TokenType::Let => cst::Statement::from(parse_let(lexer)?),

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

pub fn parse_expression(lexer: &mut Lexer) -> Result<cst::Expression> {
    match lexer.peek().get_type() {
        TokenType::Lambda => Ok(cst::Expression::from(parse_abstraction(lexer)?)),
        TokenType::LeftBracket | TokenType::Identifier(..) | TokenType::Number(..) => {
            Ok(cst::Expression::from(parse_application(lexer)?))
        }
        ref found => Err(format!("Expected '\\', '(' or identifier, found {}", found)),
    }
}

fn parse_abstraction(lexer: &mut Lexer) -> Result<cst::AbstractionExpression> {
    expect(lexer, &TokenType::Lambda)?;
    let parameters = parse_parameters(lexer)?;
    expect(lexer, &TokenType::Dot)?;
    let expression = parse_expression(lexer)?;
    Ok(cst::AbstractionExpression::new(parameters, expression))
}

fn parse_parameters(lexer: &mut Lexer) -> Result<Vec<cst::Identifier>> {
    let mut parameters = Vec::new();
    while let TokenType::Identifier(..) = lexer.peek().get_type() {
        parameters.push(parse_identifier(lexer)?);
    }
    Ok(parameters)
}

fn parse_application(lexer: &mut Lexer) -> Result<cst::ApplicationExpression> {
    let mut expressions = Vec::new();
    loop {
        expressions.push(match lexer.peek().get_type() {
            TokenType::Identifier(..) => {
                cst::Expression::from(cst::VariableExpression::new(parse_identifier(lexer)?))
            }

            TokenType::Number(..) => cst::Expression::from(parse_number(lexer)?),

            TokenType::LeftBracket => {
                expect(lexer, &TokenType::LeftBracket)?;
                let expression = parse_expression(lexer)?;
                expect(lexer, &TokenType::RightBracket)?;
                expression
            }

            TokenType::Lambda => cst::Expression::from(parse_abstraction(lexer)?),

            _ => break,
        });
    }
    Ok(cst::ApplicationExpression::new(expressions))
}

fn parse_identifier(lexer: &mut Lexer) -> Result<cst::Identifier> {
    match lexer.next().get_type() {
        TokenType::Identifier(identifier) => Ok(cst::Identifier::new(identifier.as_str())),
        found => Err(format!("Expected identifier, found {}", found)),
    }
}

fn parse_let(lexer: &mut Lexer) -> Result<cst::LetStatement> {
    expect(lexer, &TokenType::Let)?;
    let variable = parse_identifier(lexer)?;
    expect(lexer, &TokenType::Equal)?;
    let expression = parse_expression(lexer)?;
    Ok(cst::LetStatement::new(variable, expression))
}

fn parse_number(lexer: &mut Lexer) -> Result<cst::Number> {
    match lexer.next().get_type() {
        TokenType::Number(number) => Ok(cst::Number::new(number.as_str())),
        found => Err(format!("Expected number, found {}", found)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_abstraction() {
        let result = parse_abstraction(&mut Lexer::new("\\x y. x"));
        let expected = Ok(cst::AbstractionExpression::new(
            vec![cst::Identifier::new("x"), cst::Identifier::new("y")],
            cst::Expression::from(cst::ApplicationExpression::new(vec![
                cst::Expression::from(cst::VariableExpression::new(cst::Identifier::new("x"))),
            ])),
        ));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_application() {
        let result = parse_application(&mut Lexer::new("x y z"));
        let expected = Ok(cst::ApplicationExpression::new(vec![
            cst::Expression::from(cst::VariableExpression::new(cst::Identifier::new("x"))),
            cst::Expression::from(cst::VariableExpression::new(cst::Identifier::new("y"))),
            cst::Expression::from(cst::VariableExpression::new(cst::Identifier::new("z"))),
        ]));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_paratemers() {
        let result = parse_parameters(&mut Lexer::new("x y z"));
        let expected = Ok(vec![
            cst::Identifier::new("x"),
            cst::Identifier::new("y"),
            cst::Identifier::new("z"),
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_let() {
        let result = parse_let(&mut Lexer::new("let x = y"));
        let expected = Ok(cst::LetStatement::new(
            cst::Identifier::new("x"),
            cst::ApplicationExpression::new(vec![cst::Expression::from(
                cst::VariableExpression::new(cst::Identifier::new("y")),
            )]),
        ));
        assert_eq!(expected, result);
    }
}
