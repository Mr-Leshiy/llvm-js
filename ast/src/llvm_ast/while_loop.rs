use super::{Expression, VariableExpression};

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: VariableExpression,
    pub body: Vec<Expression>,
}
