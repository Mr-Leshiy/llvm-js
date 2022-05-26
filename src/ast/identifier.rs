use super::Expr;

/// Identifier - Expression type for any identifier, like "name"
pub struct Identifier {
    name: String,
}

impl Expr for Identifier {}
