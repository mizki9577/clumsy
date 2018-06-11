#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate clumsy;
extern crate wasm_bindgen;

use clumsy::expression::Expression;
use clumsy::lexer::Lexer;
use clumsy::parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(source: &str) -> String {
    parser::parse(&mut Lexer::new(source).peekable())
        .map(|ref ast| format!("{}", Expression::from(ast).evaluate()))
        .unwrap_or_else(|err| format!("{}", err))
}
