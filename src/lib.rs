#![feature(plugin, box_syntax, box_patterns, nll)]
#![cfg_attr(feature = "clippy", plugin(clippy))]

pub mod interpreter;
pub mod lexer;
pub mod parser;
