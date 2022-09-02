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
}

impl Priority for BinaryExpType {
    fn priority(&self) -> u8 {
        match self {
            BinaryExpType::And => 0,
            BinaryExpType::Or => 0,
        }
    }
}
