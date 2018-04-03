use ast;
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
        match expression(tokens) {
            Ok(expression) => result.push(expression),
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

fn expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
    match tokens.peek() {
        Some(Token::Symbol(_)) => symbol(tokens).map(ast::Expression::Symbol),
        Some(Token::LeftBracket) => list(tokens),
        found => Err(format!("Expected Symbol or '(', found {:?}", found)),
    }
}

fn symbol(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Symbol> {
    match tokens.next() {
        Some(Token::Symbol(ref symbol)) => Ok(ast::Symbol(symbol.to_string())),
        token => Err(format!("Expected Symbol, found {:?}", token)),
    }
}

fn list(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Expression> {
    expect(tokens, &Token::LeftBracket)?;

    let result = match tokens.peek() {
        Some(Token::Lambda) => function(tokens).map(ast::Expression::Function),
        Some(Token::Symbol(_)) | Some(Token::LeftBracket) => {
            application(tokens).map(ast::Expression::Application)
        }
        found => return Err(format!("Expected '\\', Symbol or '(', found {:?}", found)),
    };

    expect(tokens, &Token::RightBracket)?;
    result
}

fn function(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Function> {
    expect(tokens, &Token::Lambda)?;

    let parameter = symbol(tokens)?;
    let body = box expression(tokens)?;

    Ok(ast::Function { parameter, body })
}

fn application(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Application> {
    let callee = box expression(tokens)?;
    let argument = box expression(tokens)?;

    Ok(ast::Application { callee, argument })
}
