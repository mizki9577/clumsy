#![feature(wasm_custom_section, wasm_import_module, use_extern_macros)]

extern crate clumsy;
extern crate wasm_bindgen;

use clumsy::ast::Expression;
use clumsy::lexer::Lexer;
use clumsy::parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(source: &str) -> String {
    parser::parse(&mut Lexer::new(source))
        .map(|ref cst| format!("{}", Expression::from_cst_program(cst).evaluate()))
        .unwrap_or_else(|err| format!("{}", err))
}
