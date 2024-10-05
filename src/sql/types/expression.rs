use super::Value;

use serde::{Deserialize, Serialize};

/// An expression, made up of nested operations and values. Values are either
/// constants or dynamic column references. Evaluates to a final value during
/// query execution, using row values for column references.
///
/// Since this is a recursive data structure, we have to box each child
/// expression, which incurs a heap allocation per expression node. There are
/// clever ways to avoid this, but we keep it simple.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// A constant value.
    Constant(Value),
    /// A column reference. Used as row index when evaluating expressions.
    Column(usize),

    /// Logical AND of two booleans: a AND b.
    And(Box<Expression>, Box<Expression>),
    /// Logical OR of two booleans: a OR b.
    Or(Box<Expression>, Box<Expression>),
    /// Logical NOT of a boolean: NOT a.
    Not(Box<Expression>),

    /// Equality comparison of two values: a = b.
    Equal(Box<Expression>, Box<Expression>),
    /// Greater than comparison of two values: a > b.
    GreaterThan(Box<Expression>, Box<Expression>),
    /// Less than comparison of two values: a < b.
    LessThan(Box<Expression>, Box<Expression>),
    /// Checks for the given value: IS NULL or IS NAN.
    Is(Box<Expression>, Value),

    /// Adds two numbers: a + b.
    Add(Box<Expression>, Box<Expression>),
    /// Divides two numbers: a / b.
    Divide(Box<Expression>, Box<Expression>),
    /// Exponentiates two numbers, i.e. a ^ b.
    Exponentiate(Box<Expression>, Box<Expression>),
    /// Takes the factorial of a number: 4! = 4*3*2*1.
    Factorial(Box<Expression>),
    /// The identify function, which simply returns the same number: +a.
    Identity(Box<Expression>),
    /// Multiplies two numbers: a * b.
    Multiply(Box<Expression>, Box<Expression>),
    /// Negates the given number: -a.
    Negate(Box<Expression>),
    /// The remainder after dividing two numbers: a % b.
    Remainder(Box<Expression>, Box<Expression>),
    /// Takes the square root of a number: √a.
    SquareRoot(Box<Expression>),
    /// Subtracts two numbers: a - b.
    Subtract(Box<Expression>, Box<Expression>),

    // Checks if a string matches a pattern: a LIKE b.
    Like(Box<Expression>, Box<Expression>),
}

impl From<Value> for Expression {
    fn from(value: Value) -> Self {
        Expression::Constant(value)
    }
}

impl From<Value> for Box<Expression> {
    fn from(value: Value) -> Self {
        Box::new(value.into())
    }
}
