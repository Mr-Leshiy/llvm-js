use super::VariableExpression;

#[derive(Clone, Debug, PartialEq)]
pub enum LogicalExpression {
    Not(VariableExpression),
    And {
        left: VariableExpression,
        right: VariableExpression,
    },
    Or {
        left: VariableExpression,
        right: VariableExpression,
    },
}
