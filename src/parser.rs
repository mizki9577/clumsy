use ast;
use lexer::{Lexer, TokenType};
use std::result;

type Result<T> = result::Result<T, String>;

fn expect(tokens: &mut Lexer, expected: &TokenType) -> Result<()> {
    match tokens.next() {
        ref found if expected == &found.token_type => Ok(()),
        found => Err(format!("Expected {}, found {}", expected, found)),
    }
}

pub fn parse(tokens: &mut Lexer) -> Result<ast::Program> {
    let mut statements = Vec::new();

    while let TokenType::Lambda
    | TokenType::LeftBracket
    | TokenType::Let
    | TokenType::Identifier(..) = tokens.peek().token_type
    {
        statements.push(parse_statement(tokens)?)
    }

    Ok(ast::Program(statements))
}

fn parse_statement(tokens: &mut Lexer) -> Result<ast::Statement> {
    let result = match tokens.peek().token_type {
        TokenType::Lambda | TokenType::LeftBracket | TokenType::Identifier(..) => {
            ast::Statement::from(ast::ExpressionStatement::new(parse_expression(tokens)?))
        }

        TokenType::Let => ast::Statement::from(parse_let(tokens)?),

        ref found => {
            return Err(format!(
                "Expected '\\', '(', 'let' or Variable, found {}",
                found
            ))
        }
    };
    expect(tokens, &TokenType::Semicolon)?;
    Ok(result)
}

pub fn parse_expression(tokens: &mut Lexer) -> Result<ast::Expression> {
    match tokens.peek().token_type {
        TokenType::Lambda => Ok(ast::Expression::from(parse_abstraction(tokens)?)),
        TokenType::LeftBracket | TokenType::Identifier(..) => {
            Ok(ast::Expression::from(parse_application(tokens)?))
        }
        ref found => Err(format!("Expected '\\', '(' or Variable, found {}", found)),
    }
}

fn parse_abstraction(tokens: &mut Lexer) -> Result<ast::AbstractionExpression> {
    expect(tokens, &TokenType::Lambda)?;
    let parameters = parse_parameters(tokens)?;
    expect(tokens, &TokenType::Dot)?;
    let expression = parse_expression(tokens)?;
    Ok(ast::AbstractionExpression::new(parameters, expression))
}

fn parse_parameters(tokens: &mut Lexer) -> Result<Vec<ast::Identifier>> {
    let mut parameters = Vec::new();
    while let TokenType::Identifier(..) = tokens.peek().token_type {
        parameters.push(parse_identifier(tokens)?);
    }
    Ok(parameters)
}

fn parse_application(tokens: &mut Lexer) -> Result<ast::ApplicationExpression> {
    let mut expressions = Vec::new();
    loop {
        expressions.push(match tokens.peek().token_type {
            TokenType::Identifier(..) => {
                ast::Expression::from(ast::VariableExpression::new(parse_identifier(tokens)?))
            }

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

fn parse_identifier(tokens: &mut Lexer) -> Result<ast::Identifier> {
    match tokens.next().token_type {
        TokenType::Identifier(variable) => Ok(ast::Identifier::new(variable)),
        found => Err(format!("Expected Variable, found {}", found)),
    }
}

fn parse_let(tokens: &mut Lexer) -> Result<ast::LetStatement> {
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
