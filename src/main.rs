#![feature(plugin, box_syntax, nll, if_while_or_patterns)]
#![cfg_attr(test, plugin(clippy))]

extern crate rustyline;

mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use rustyline::error::ReadlineError;

static PROMPT: &str = ">>> ";

fn main() {
    let mut rl = rustyline::Editor::<()>::new().history_ignore_dups(true);
    loop {
        match rl.readline(PROMPT) {
            Ok(source) => {
                rl.add_history_entry(&source);
                let result = eval(&source);
                println!("{}", result);
            }
            Err(ReadlineError::Eof) => break,
            Err(error) => println!("{}", error),
        }
    }
}

fn eval(source: &str) -> String {
    let tokens = Lexer::new(source);
    let ast = parser::parse(tokens);
    format!("{:#?}", ast)
}
