#![feature(box_syntax)]

extern crate rustyline;

mod ast;
mod lexer;
mod parser;

static PROMPT: &str = ">>> ";

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(source) = rl.readline(PROMPT) {
        let result = eval(&source);
        println!("{}", result);
    }
}

fn eval(source: &str) -> String {
    let tokens = lexer::tokenize(source);
    let ast = parser::parse(&tokens);
    format!("{:?}", ast)
}
