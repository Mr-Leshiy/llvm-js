use super::{AssigmentExpression, VariableDeclaration};

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    AssigmentExpression(AssigmentExpression),
}

/// Program
pub struct Program {
    pub body: Vec<Expression>,
}
