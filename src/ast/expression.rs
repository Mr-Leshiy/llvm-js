use super::{AssigmentExpression, VariableDeclaration};

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    AssigmentExpression(AssigmentExpression),
    BlockStatement { body: Vec<Expression> },
}
