#![feature(plugin, box_syntax, nll)]
#![cfg_attr(check, plugin(clippy))]

pub mod lexer;
pub mod parser;
pub mod translator;
mod utils;
