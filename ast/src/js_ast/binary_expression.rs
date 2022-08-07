use super::VariableExpression;

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryExpression {
    And {
        left: VariableExpression,
        right: VariableExpression,
    },
    Or {
        left: VariableExpression,
        right: VariableExpression,
    },
}
