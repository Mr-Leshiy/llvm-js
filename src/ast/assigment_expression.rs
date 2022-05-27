use super::{Expr, Identifier, RightAssigmentValue};

/// AssigmentExpression - Expression type for variable assigment, like "a = 4"
pub struct AssigmentExpression {
    pub left: Identifier,
    pub right: RightAssigmentValue,
}

impl Expr for AssigmentExpression {}
