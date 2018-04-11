mod ast;

use lexer::Token;
use std::iter::Peekable;
use std::result;

type Result<T> = result::Result<T, String>;

pub fn parse(tokens: impl Iterator<Item = Token>) -> Result<ast::Program> {
    let mut tokens = tokens.peekable();
    program(&mut tokens)
}

fn expect(tokens: &mut Peekable<impl Iterator<Item = Token>>, expected: &Token) -> Result<()> {
    match tokens.next() {
        Some(ref found) if expected == found => Ok(()),
        found => Err(format!("Expected {:?}, found {:?}", expected, found)),
    }
}

fn program(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Program> {
    let mut result = Vec::new();
    while let Some(_) = tokens.peek() {
        result.push(expression(tokens)?);
    }
    Ok(result)
}

fn expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
    match tokens.peek() {
        Some(Token::Lambda) => abstraction(tokens),
        Some(Token::LeftBracket) | Some(Token::Variable(_)) => application(tokens),
        found => Err(format!("Expected '\\', '(' or Variable, found {:?}", found)),
    }
}

fn abstraction(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
    expect(tokens, &Token::Lambda)?;
    let variables = variables(tokens)?;
    expect(tokens, &Token::Dot)?;
    let expression = box expression(tokens)?;
    Ok(ast::Expression::Abstraction {
        variables,
        expression,
    })
}

fn application(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
    let mut items = Vec::new();
    while let Some(Token::LeftBracket) | Some(Token::Variable(_)) = tokens.peek() {
        items.push(item(tokens)?);
    }
    Ok(ast::Expression::Application { items })
}

fn item(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Item> {
    match tokens.peek() {
        Some(Token::Variable(_)) => variable(tokens).map(ast::Item::Variable),
        Some(Token::LeftBracket) => {
            expect(tokens, &Token::LeftBracket)?;
            let result = expression(tokens).map(ast::Item::Expression)?;
            expect(tokens, &Token::RightBracket)?;
            Ok(result)
        }
        found => Err(format!("Expected Variable or '(', found {:?}", found)),
    }
}

fn variables(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Variables> {
    let mut variables = Vec::new();
    while let Some(Token::Variable(_)) = tokens.peek() {
        variables.push(variable(tokens)?);
    }
    Ok(variables)
}

fn variable(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Variable> {
    match tokens.next() {
        Some(Token::Variable(ref variable)) => Ok(ast::Variable(variable.to_string())),
        token => Err(format!("Expected Variable, found {:?}", token)),
    }
}
