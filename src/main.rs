#![feature(box_syntax)]

extern crate rustyline;

mod ast;
mod lexer;
mod parser;

static PROMPT: &'static str = ">>> ";

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(source) = rl.readline(PROMPT) {
        eval(&source);
    }
}

fn eval(source: &str) {
    println!("{:?}", source);

    let tokens = lexer::tokenize(source);
    println!("{:?}", tokens);

    let ast = parser::parse(&tokens);
    println!("{:?}", ast);
}
