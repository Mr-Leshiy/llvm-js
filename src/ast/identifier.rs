use super::Expr;

/// Identifier - Expression type for any identifier, like "name"
#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl Expr for Identifier {}
