//! # Parser
//!
//! Parses raw SQL strings into a structured Abstract Syntax Tree.

pub mod ast;
pub mod lexer;
pub mod the_parser;

pub use lexer::{is_ident, Keyword, Lexer, Token};
pub use the_parser::Parser;
