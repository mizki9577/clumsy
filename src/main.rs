#![feature(box_syntax)]

extern crate rustyline;

mod ast;
mod lexer;
mod parser;

use lexer::Lexer;

static PROMPT: &str = ">>> ";

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(source) = rl.readline(PROMPT) {
        let result = eval(&source);
        println!("{}", result);
    }
}

fn eval(source: &str) -> String {
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let ast = parser::parse(&tokens);
    format!("{:?}", ast)
}
