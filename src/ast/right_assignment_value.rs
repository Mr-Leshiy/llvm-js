use super::{Identifier, Literal};

/// RightAssigmentValue
#[derive(Debug, PartialEq)]
pub enum RightAssigmentValue {
    Literal(Literal),
    Identifier(Identifier),
}
