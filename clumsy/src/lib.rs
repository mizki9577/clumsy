#![feature(plugin, box_syntax, nll, if_while_or_patterns)]
#![cfg_attr(check, plugin(clippy))]

pub mod lexer;
pub mod parser;
pub mod translator;
mod utils;
