#![feature(underscore_imports)]

#[macro_use]
extern crate structopt;
extern crate ansi_term;
extern crate clumsy;
extern crate rustyline;

use ansi_term::Color;
use clumsy::interpreter::Expression;
use clumsy::lexer::Lexer;
use clumsy::parser;
use rustyline::error::ReadlineError;
use std::fs::File;
use std::io::BufReader;
use std::io::Read as _;
use std::path::PathBuf;
use structopt::StructOpt;

static PROMPT: &str = ">>> ";
static RED: Color = Color::Fixed(9);

#[derive(StructOpt)]
#[structopt(name = "Clumsy")]
struct Options {
    #[structopt(value_name = "file", parse(from_os_str), conflicts_with = "expression")]
    program: Option<PathBuf>,

    #[structopt(short = "e", value_name = "expression", conflicts_with = "program")]
    expression: Option<String>,

    #[structopt(long = "history", value_name = "file", env = "CLUMSY_HISTORY", parse(from_os_str))]
    history: Option<PathBuf>,
}

fn main() {
    match Options::from_args() {
        Options {
            expression: Some(ref expression),
            ..
        } => eval(expression),

        Options {
            program: Some(path),
            ..
        } => {
            match File::open(path) {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    let mut source = String::new();
                    if reader.read_to_string(&mut source).is_ok() {
                        eval(&source);
                    } else {
                        // TODO: handle errors
                    }
                }
                Err(error) => println!("{}", error),
            }
        }

        Options { ref history, .. } => repl(history),
    }
}

fn repl(history: &Option<PathBuf>) {
    let mut rl = rustyline::Editor::<()>::new().history_ignore_dups(true);
    if let Some(history) = history {
        let _ = rl.load_history(history);
    }

    loop {
        match rl.readline(PROMPT) {
            Ok(source) => {
                rl.add_history_entry(&source);
                eval(&source);
            }
            Err(ReadlineError::Eof) => break,
            Err(error) => println!("{}", RED.paint(error.to_string())),
        }
    }

    if let Some(history) = history {
        let _ = rl.save_history(history);
    }
}

fn eval(source: &str) {
    let tokens = &mut Lexer::new(source).peekable();
    match parser::parse_expression(tokens).map(|ast| {
        let expression = Expression::from(&ast);
        expression.evaluate()
    }) {
        Ok(result) => println!("{}", result),
        Err(error) => println!("{}", RED.paint(error)),
    };
}
