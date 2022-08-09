use super::VariableExpression;

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression {
    pub exp: VariableExpression,
    pub exp_type: UnaryExpType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryExpType {
    Not,
}
