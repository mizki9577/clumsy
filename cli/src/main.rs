#![feature(plugin)]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate ansi_term;
extern crate clumsy;
extern crate rustyline;

use ansi_term::Color;
use clumsy::lexer::Lexer;
use clumsy::parser;
use clumsy::translator::Expression;
use rustyline::error::ReadlineError;

static PROMPT: &str = ">>> ";
static RED: Color = Color::Fixed(9);

fn main() {
    let mut rl = rustyline::Editor::<()>::new().history_ignore_dups(true);
    loop {
        match rl.readline(PROMPT) {
            Ok(source) => {
                rl.add_history_entry(&source);
                let result = eval(&source);
                match result {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("{}", RED.paint(error)),
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(error) => println!("{}", RED.paint(error.to_string())),
        }
    }
}

fn eval(source: &str) -> Result<String, String> {
    let mut tokens = Lexer::new(source).peekable();
    let ast = parser::parse_expression(&mut tokens)?;
    Ok(format!("{}", Expression::from_ast(&ast)))
}
