use super::{DataType, Value};
use crate::encoding;
use crate::errinput;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A table schema, which specifies its data structure and constraints.
///
/// Tables can't change after they are created. There is no ALTER TABLE nor
/// CREATE/DROP INDEX -- only CREATE TABLE and DROP TABLE.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Table {
    /// The table name. Can't be empty.
    pub name: String,
    /// The primary key column index. A table must have a primary key, and it
    /// can only be a single column.
    pub primary_key: usize,
    /// The table's columns. Must have at least one.
    pub columns: Vec<Column>,
}

impl encoding::Value for Table {}

/// A table column.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Column {
    /// Column name. Can't be empty.
    pub name: String,
    /// Column datatype.
    pub datatype: DataType,
    /// Whether the column allows null values. Not legal for primary keys.
    pub nullable: bool,
    /// The column's default value. If None, the user must specify an explicit
    /// value. Must match the column datatype. Nullable columns require a
    /// default (often Null), and Null is only a valid default when nullable.
    pub default: Option<Value>,
    /// Whether the column should only allow unique values (ignoring NULLs).
    /// Must be true for a primary key column.
    pub unique: bool,
    /// Whether the column should have a secondary index. Must be false for
    /// primary keys, which are the implicit primary index. Must be true for
    /// unique or reference columns.
    pub index: bool,
    /// If set, this column is a foreign key reference to the given table's
    /// primary key. Must be of the same type as the target primary key.
    pub references: Option<String>,
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CREATE TABLE {} (", format_ident(&self.name))?;
        for (i, column) in self.columns.iter().enumerate() {
            write!(f, "  {} {}", format_ident(&column.name), column.datatype)?;
            if i == self.primary_key {
                write!(f, " PRIMARY KEY")?;
            } else if !column.nullable {
                write!(f, " NOT NULL")?;
            }
            if let Some(default) = &column.default {
                write!(f, " DEFAULT {default}")?;
            }
            if i != self.primary_key {
                if column.unique {
                    write!(f, " UNIQUE")?;
                }
                if column.index {
                    write!(f, " INDEX")?;
                }
            }
            if let Some(reference) = &column.references {
                write!(f, " REFERENCES {reference}")?;
            }
            if i < self.columns.len() - 1 {
                write!(f, ",")?;
            }
            writeln!(f)?;
        }
        write!(f, ")")
    }
}

/// Formats an identifier as valid SQL, quoting it if necessary.
fn format_ident(ident: &str) -> Cow<str> {
    if crate::sql::parser::is_ident(ident) {
        return ident.into();
    }
    format!("\"{}\"", ident.replace('\"', "\"\"")).into()
}
