use serde::{Deserialize, Serialize};

/// A primitive SQL value.
///
/// For simplicity, only a handful of representative scalar types are supported,
/// no compound types or more compact variants.
///
/// In SQL, neither Null nor floating point NaN are considered equal to
/// themselves (they are unknown values). However, in code, we consider them
/// equal and comparable. This is necessary to allow sorting and processing of
/// these values (e.g. in index lookups, aggregation buckets, etc.).
///
/// SQL expression evaluation have special handling of these values to produce the
/// desired NULL != NULL and NAN != NAN semantics in SQL queries.
///
/// Float -0.0 is considered equal to 0.0. It is normalized to 0.0 when stored.
/// Similarly, -NaN is normalized to NaN.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    /// An unknown value of unknown type.
    Null,
    /// A boolean.
    Boolean(bool),
    /// A 64-bit signed integer.
    Integer(i64),
    /// A 64-bit floating point number.
    Float(f64),
    /// A UTF-8 encoded string.
    String(String),
}
