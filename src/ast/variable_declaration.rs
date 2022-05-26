use super::{Expr, Identifier, RightAssigmentValue};

/// VariableDeclaration - Expression type for variable declaration, like "var a = 4"
pub struct VariableDeclaration {
    id: Identifier,
    init: RightAssigmentValue,
}

impl Expr for VariableDeclaration {}
