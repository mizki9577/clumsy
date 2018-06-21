use ast;
use lexer::{Lexer, TokenType};
use std::result;

type Result<T> = result::Result<T, String>;

fn expect(lexer: &mut Lexer, expected: &TokenType) -> Result<()> {
    match lexer.next() {
        ref found if expected == found.get_type() => Ok(()),
        found => Err(format!("Expected {}, found {}", expected, found)),
    }
}

pub fn parse(lexer: &mut Lexer) -> Result<ast::Program> {
    let mut statements = Vec::new();

    while let TokenType::Lambda
    | TokenType::LeftBracket
    | TokenType::Let
    | TokenType::Identifier(..) = lexer.peek().get_type()
    {
        statements.push(parse_statement(lexer)?)
    }

    Ok(ast::Program(statements))
}

fn parse_statement(lexer: &mut Lexer) -> Result<ast::Statement> {
    let result = match lexer.peek().get_type() {
        TokenType::Lambda
        | TokenType::LeftBracket
        | TokenType::Identifier(..)
        | TokenType::Number(..) => {
            ast::Statement::from(ast::ExpressionStatement::new(parse_expression(lexer)?))
        }

        TokenType::Let => ast::Statement::from(parse_let(lexer)?),

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

pub fn parse_expression(lexer: &mut Lexer) -> Result<ast::Expression> {
    match lexer.peek().get_type() {
        TokenType::Lambda => Ok(ast::Expression::from(parse_abstraction(lexer)?)),
        TokenType::LeftBracket | TokenType::Identifier(..) => {
            Ok(ast::Expression::from(parse_application(lexer)?))
        }
        TokenType::Number(..) => Ok(ast::Expression::from(parse_number(lexer)?)),
        ref found => Err(format!("Expected '\\', '(' or identifier, found {}", found)),
    }
}

fn parse_abstraction(lexer: &mut Lexer) -> Result<ast::AbstractionExpression> {
    expect(lexer, &TokenType::Lambda)?;
    let parameters = parse_parameters(lexer)?;
    expect(lexer, &TokenType::Dot)?;
    let expression = parse_expression(lexer)?;
    Ok(ast::AbstractionExpression::new(parameters, expression))
}

fn parse_parameters(lexer: &mut Lexer) -> Result<Vec<ast::Identifier>> {
    let mut parameters = Vec::new();
    while let TokenType::Identifier(..) = lexer.peek().get_type() {
        parameters.push(parse_identifier(lexer)?);
    }
    Ok(parameters)
}

fn parse_application(lexer: &mut Lexer) -> Result<ast::ApplicationExpression> {
    let mut expressions = Vec::new();
    loop {
        expressions.push(match lexer.peek().get_type() {
            TokenType::Identifier(..) => {
                ast::Expression::from(ast::VariableExpression::new(parse_identifier(lexer)?))
            }

            TokenType::Number(..) => ast::Expression::from(parse_number(lexer)?),

            TokenType::LeftBracket => {
                expect(lexer, &TokenType::LeftBracket)?;
                let expression = parse_expression(lexer)?;
                expect(lexer, &TokenType::RightBracket)?;
                expression
            }

            TokenType::Lambda => ast::Expression::from(parse_abstraction(lexer)?),

            _ => break,
        });
    }
    Ok(ast::ApplicationExpression::new(expressions))
}

fn parse_identifier(lexer: &mut Lexer) -> Result<ast::Identifier> {
    match lexer.next().get_type() {
        TokenType::Identifier(identifier) => Ok(ast::Identifier::new(identifier.as_str())),
        found => Err(format!("Expected identifier, found {}", found)),
    }
}

fn parse_let(lexer: &mut Lexer) -> Result<ast::LetStatement> {
    expect(lexer, &TokenType::Let)?;
    let variable = parse_identifier(lexer)?;
    expect(lexer, &TokenType::Equal)?;
    let expression = parse_expression(lexer)?;
    Ok(ast::LetStatement::new(variable, expression))
}

fn parse_number(lexer: &mut Lexer) -> Result<ast::Number> {
    match lexer.next().get_type() {
        TokenType::Number(number) => Ok(ast::Number::new(number.as_str())),
        found => Err(format!("Expected number, found {}", found)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_abstraction() {
        let result = parse_abstraction(&mut Lexer::new("\\x y. x"));
        let expected = Ok(ast::AbstractionExpression::new(
            vec![ast::Identifier::new("x"), ast::Identifier::new("y")],
            ast::Expression::from(ast::ApplicationExpression::new(vec![
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("x"))),
            ])),
        ));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_application() {
        let result = parse_application(&mut Lexer::new("x y z"));
        let expected = Ok(ast::ApplicationExpression::new(vec![
            ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("x"))),
            ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("y"))),
            ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("z"))),
        ]));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_paratemers() {
        let result = parse_parameters(&mut Lexer::new("x y z"));
        let expected = Ok(vec![
            ast::Identifier::new("x"),
            ast::Identifier::new("y"),
            ast::Identifier::new("z"),
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_let() {
        let result = parse_let(&mut Lexer::new("let x = y"));
        let expected = Ok(ast::LetStatement::new(
            ast::Identifier::new("x"),
            ast::ApplicationExpression::new(vec![ast::Expression::from(
                ast::VariableExpression::new(ast::Identifier::new("y")),
            )]),
        ));
        assert_eq!(expected, result);
    }
}
