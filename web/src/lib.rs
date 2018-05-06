#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate clumsy;
extern crate wasm_bindgen;

use clumsy::lexer::Lexer;
use clumsy::parser;
use clumsy::translator::Expression;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(source: &str) -> String {
    format!(
        "{:#?}",
        parser::parse_expression(&mut Lexer::new(source).peekable())
            .map(|ref ast| Expression::from_ast(ast))
    )
}
