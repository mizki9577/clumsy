pub mod ast;
#[cfg(test)]
mod tests;

use lexer::Token;
use std::iter::Peekable;
use std::result;

type Result<T> = result::Result<T, String>;

fn expect(tokens: &mut Peekable<impl Iterator<Item = Token>>, expected: &Token) -> Result<()> {
    match tokens.next() {
        Some(ref found) if expected == found => Ok(()),
        found => Err(format!("Expected {:?}, found {:?}", expected, found)),
    }
}

pub fn parse_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::Expression> {
    match tokens.peek() {
        Some(Token::Lambda) => parse_abstraction(tokens),
        Some(Token::LeftBracket) | Some(Token::Variable(_)) => parse_application(tokens),
        found => Err(format!("Expected '\\', '(' or Variable, found {:?}", found)),
    }
}

fn parse_abstraction(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::Expression> {
    expect(tokens, &Token::Lambda)?;
    parse_abstraction_body(tokens)
}

fn parse_abstraction_body(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::Expression> {
    let parameter = parse_variable(tokens)?;
    let expression = box match tokens.peek() {
        Some(Token::Variable(_)) => parse_abstraction_body(tokens)?,
        Some(Token::Dot) => {
            expect(tokens, &Token::Dot)?;
            parse_expression(tokens)?
        }
        found => return Err(format!("Expected '.' or Variable, found {:?}", found)),
    };
    Ok(ast::Expression::Abstraction {
        parameter,
        expression,
    })
}

fn parse_application(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ast::Expression> {
    let mut items = Vec::new();
    loop {
        items.push(match tokens.peek() {
            Some(Token::Variable(_)) => parse_variable(tokens).map(ast::Expression::Variable)?,
            Some(Token::LeftBracket) => {
                expect(tokens, &Token::LeftBracket)?;
                let result = parse_expression(tokens)?;
                expect(tokens, &Token::RightBracket)?;
                result
            }
            _ => break,
        });
    }
    Ok(fix_application(items))
}

fn fix_application(mut items: Vec<ast::Expression>) -> ast::Expression {
    let last = items.pop().expect("Application list is empty!");
    if items.is_empty() {
        last
    } else {
        ast::Expression::Application {
            callee: box fix_application(items),
            argument: box last,
        }
    }
}

fn parse_variable(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Variable> {
    match tokens.next() {
        Some(Token::Variable(ref variable)) => Ok(ast::Variable::new(variable)),
        token => Err(format!("Expected Variable, found {:?}", token)),
    }
}
