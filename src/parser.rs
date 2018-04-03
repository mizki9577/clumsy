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
    if let Some(token) = tokens.peek() {
        match token {
            Token::Symbol(_) => symbol(tokens).map(ast::Expression::Symbol),
            Token::LeftBracket => list(tokens),
            found => Err(format!("Expected Symbol or '(', found {:?}", found)),
        }
    } else {
        Err("Expected Symbol or '(', found None".to_string())
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

    let result = if let Some(token) = tokens.peek() {
        match token {
            Token::Lambda => function(tokens).map(ast::Expression::Function),
            Token::Symbol(_) | Token::LeftBracket => {
                application(tokens).map(ast::Expression::Application)
            }
            found => Err(format!("Expected '\\', Symbol or '(', found {:?}", found)),
        }
    } else {
        Err("Expected '\\' or Symbol, found EOF".to_string())
    }?;

    expect(tokens, &Token::RightBracket)?;
    Ok(result)
}

fn function(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Function> {
    expect(tokens, &Token::Lambda)?;

    let parameter = symbol(tokens)?;
    let body = expression(tokens)?;

    Ok(ast::Function {
        parameter,
        body: box body,
    })
}

fn application(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<ast::Application> {
    let callee = expression(tokens)?;
    let argument = expression(tokens)?;

    Ok(ast::Application {
        callee: box callee,
        argument: box argument,
    })
}
