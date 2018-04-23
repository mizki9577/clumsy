#![feature(plugin, box_syntax, nll, if_while_or_patterns)]
#![cfg_attr(check, plugin(clippy))]

extern crate ansi_term;
extern crate clumsy;
extern crate rustyline;

use ansi_term::Color;
use clumsy::lexer::Lexer;
use clumsy::parser;
use clumsy::DeBruijnIndex;
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
    let tokens = Lexer::new(source);
    let ast = parser::parse(tokens)?;
    let dbi = DeBruijnIndex::from_ast(&ast);
    Ok(format!("{:#?}", dbi))
}
