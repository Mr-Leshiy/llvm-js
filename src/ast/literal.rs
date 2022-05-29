use super::Expr;

/// Literal - Expression type for numeric literals like "1.0" or string literals "hello".
#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
}

impl Expr for Literal {}
