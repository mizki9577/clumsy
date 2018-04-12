mod ast;

use lexer::Token;
use std::collections::VecDeque;
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

    let mut variables = VecDeque::new();
    while let Some(Token::Variable(_)) = tokens.peek() {
        variables.push_back(variable(tokens)?);
    }

    expect(tokens, &Token::Dot)?;
    let expression = expression(tokens)?;
    Ok(fix_abstraction(variables, expression))
}

fn fix_abstraction(
    mut variables: VecDeque<ast::Variable>,
    expression: ast::Expression,
) -> ast::Expression {
    ast::Expression::Abstraction {
        parameter: variables.pop_front().expect("Parameter list is empty!"),
        expression: box if variables.is_empty() {
            expression
        } else {
            fix_abstraction(variables, expression)
        },
    }
}

fn application(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
    let mut items = VecDeque::new();
    loop {
        items.push_back(match tokens.peek() {
            Some(Token::Variable(_)) => variable(tokens).map(ast::Expression::Variable)?,
            Some(Token::LeftBracket) => {
                expect(tokens, &Token::LeftBracket)?;
                let result = expression(tokens)?;
                expect(tokens, &Token::RightBracket)?;
                result
            }
            _ => break,
        });
    }
    Ok(fix_application(items))
}

fn fix_application(mut items: VecDeque<ast::Expression>) -> ast::Expression {
    let head = items.pop_front().expect("Application list is empty!");
    if items.is_empty() {
        head
    } else {
        ast::Expression::Application {
            callee: box head,
            argument: box fix_application(items),
        }
    }
}

fn variable(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Variable> {
    match tokens.next() {
        Some(Token::Variable(ref variable)) => Ok(ast::Variable(variable.to_string())),
        token => Err(format!("Expected Variable, found {:?}", token)),
    }
}
