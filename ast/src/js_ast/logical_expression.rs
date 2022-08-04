use super::VariableValue;

#[derive(Clone, Debug, PartialEq)]
pub enum LogicalExpression {
    Not(VariableValue),
    And {
        left: VariableValue,
        right: VariableValue,
    },
    Or {
        left: VariableValue,
        right: VariableValue,
    },
}
