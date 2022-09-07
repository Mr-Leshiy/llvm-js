use super::VariableExpression;
use precompiler::rpn::input::Priority;

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: VariableExpression,
    pub right: VariableExpression,
    pub op_type: BinaryExpType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryExpType {
    And,
    Or,
    Eq,
    Ne,
    SEq,
    SNe,
}

impl Priority for BinaryExpType {
    fn priority(&self) -> u8 {
        match self {
            BinaryExpType::Eq => 8,
            BinaryExpType::Ne => 8,
            BinaryExpType::SEq => 8,
            BinaryExpType::SNe => 8,
            BinaryExpType::And => 4,
            BinaryExpType::Or => 3,
        }
    }
}
