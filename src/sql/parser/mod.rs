//! # Parser
//!
//! Parses raw SQL strings into a structured Abstract Syntax Tree.

#![allow(clippy::module_inception)]

pub mod ast;
pub mod lexer;
pub mod parser;

pub use lexer::{is_ident, Keyword, Lexer, Token};
pub use parser::Parser;
