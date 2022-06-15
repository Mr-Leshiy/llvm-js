use super::{AssigmentExpression, VariableDeclaration};

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    Assigment(AssigmentExpression),
    BlockStatement { body: Vec<Expression> },
}
