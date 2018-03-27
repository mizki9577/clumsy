use ast;
use lexer::Token;
use std::iter::Peekable;
use std::result;
use std::slice::Iter;

type Tokens<'a> = Peekable<Iter<'a, Token>>;
type Result<T> = result::Result<T, String>;

pub fn parse(tokens: &[Token]) -> Result<ast::Program> {
    let mut tokens = tokens.iter().peekable();
    program(&mut tokens)
}

fn expect(tokens: &mut Tokens, expected: &Token) -> Result<()> {
    let result = match tokens.peek() {
        Some(&actual) if actual == expected => Ok(()),
        actual => Err(format!("Expected {:?}, found {:?}", expected, actual)),
    };

    if result.is_ok() {
        tokens.next();
    }
    result
}

fn program(tokens: &mut Tokens) -> Result<ast::Program> {
    let mut result = Vec::new();
    while let Ok(e) = expression(tokens) {
        result.push(e);
    }

    match tokens.peek() {
        None => Ok(result),
        Some(c) => Err(format!("Expected EOF, found {:?}", c)),
    }
}

fn expression(tokens: &mut Tokens) -> Result<ast::Expression> {
    let result = symbol(tokens).map(ast::Expression::Symbol);
    if result.is_ok() {
        result
    } else {
        list(tokens)
    }
}

fn list(tokens: &mut Tokens) -> Result<ast::Expression> {
    expect(tokens, &Token::LeftBracket)?;

    let result = function(tokens)
        .map(ast::Expression::Function)
        .or_else(|_| application(tokens).map(ast::Expression::Application))?;

    expect(tokens, &Token::RightBracket)?;
    Ok(result)
}

fn symbol(tokens: &mut Tokens) -> Result<ast::Symbol> {
    let result = match tokens.peek() {
        Some(&&Token::Symbol(ref symbol)) => Ok(ast::Symbol(symbol.to_string())),
        token => Err(format!("Expected Symbol, found {:?}", token)),
    };

    if result.is_ok() {
        tokens.next();
    }
    result
}

fn function(tokens: &mut Tokens) -> Result<ast::Function> {
    expect(tokens, &Token::Lambda)?;

    let parameter = symbol(tokens);
    let body = expression(tokens);

    match (parameter, body) {
        (Ok(parameter), Ok(body)) => Ok(ast::Function {
            parameter,
            body: box body,
        }),
        _ => Err(format!("hoge")),
    }
}

fn application(tokens: &mut Tokens) -> Result<ast::Application> {
    let callee = expression(tokens);
    let argument = expression(tokens);

    match (callee, argument) {
        (Ok(callee), Ok(argument)) => Ok(ast::Application {
            callee: box callee,
            argument: box argument,
        }),
        _ => Err(format!("fuga")),
    }
}
