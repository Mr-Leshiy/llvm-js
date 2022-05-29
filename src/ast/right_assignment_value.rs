use super::{Expr, Identifier, Literal};

/// RightAssigmentValue
#[derive(Debug, PartialEq)]
pub enum RightAssigmentValue {
    Literal(Literal),
    Identifier(Identifier),
}

impl Expr for RightAssigmentValue {}
