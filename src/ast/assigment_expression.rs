use super::{Identifier, RightAssigmentValue};

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4"
#[derive(Debug, PartialEq)]
pub struct VariableDeclaration(pub AssigmentExpression);

/// AssigmentExpression - Expression type for variable assigment, like "a = 4"
#[derive(Debug, PartialEq)]
pub struct AssigmentExpression {
    pub left: Identifier,
    pub right: RightAssigmentValue,
}
