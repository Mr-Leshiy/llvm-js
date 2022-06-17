use super::{AssigmentExpression, BlockStatement, VariableDeclaration};

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    Assigment(AssigmentExpression),
    BlockStatement(BlockStatement),
}
