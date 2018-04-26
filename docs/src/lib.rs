#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate clumsy;
extern crate wasm_bindgen;

use clumsy::lexer::Lexer;
use clumsy::parser;
use clumsy::translator::DeBruijnIndex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(source: &str) -> String {
    format!(
        "{:#?}",
        parser::parse(Lexer::new(source)).map(|ref ast| DeBruijnIndex::from_ast(ast))
    )
}
