mod ast;
#[cfg(test)]
mod tests;

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
    abstraction_body(tokens)
}

fn abstraction_body(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
    let parameter = variable(tokens)?;
    let expression = box match tokens.peek() {
        Some(Token::Variable(_)) => abstraction_body(tokens)?,
        Some(Token::Dot) => {
            expect(tokens, &Token::Dot)?;
            expression(tokens)?
        }
        found => return Err(format!("Expected '.' or Variable, found {:?}", found)),
    };
    Ok(ast::Expression::Abstraction {
        parameter,
        expression,
    })
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
    let last = items.pop_back().expect("Application list is empty!");
    if items.is_empty() {
        last
    } else {
        ast::Expression::Application {
            callee: box fix_application(items),
            argument: box last,
        }
    }
}

fn variable(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Variable> {
    match tokens.next() {
        Some(Token::Variable(ref variable)) => Ok(ast::Variable::new(variable)),
        token => Err(format!("Expected Variable, found {:?}", token)),
    }
}
