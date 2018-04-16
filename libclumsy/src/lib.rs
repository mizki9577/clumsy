#![feature(plugin, box_syntax, nll, if_while_or_patterns)]
#![cfg_attr(test, plugin(clippy))]

pub mod lexer;
pub mod parser;
