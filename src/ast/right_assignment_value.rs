use super::{Expr, Identifier, Literal};

/// RightAssigmentValue
pub enum RightAssigmentValue {
    Literal(Literal),
    Identifier(Identifier),
}

impl Expr for RightAssigmentValue {}
