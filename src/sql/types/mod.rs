//! # Types
//!
//! The SQL data model, including data types, expressions, and schema objects.

mod expression;
mod schema;
mod value;

pub use expression::Expression;
pub use value::{DataType, Label, Row, Rows, Value};
