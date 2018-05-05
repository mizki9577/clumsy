#![feature(plugin, box_syntax, nll)]
#![cfg_attr(feature = "clippy", plugin(clippy))]

pub mod lexer;
pub mod parser;
pub mod translator;
