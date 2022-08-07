use super::VariableExpression;

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryExpression {
    Not(VariableExpression),
}
