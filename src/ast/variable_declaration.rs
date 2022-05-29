use super::{Expr, Identifier, RightAssigmentValue};

/// VariableDeclaration - Expression type for variable declaration, like "var a = 4"
#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub id: Identifier,
    pub init: RightAssigmentValue,
}

impl Expr for VariableDeclaration {}
