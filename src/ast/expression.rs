use super::AssigmentExpression;

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(AssigmentExpression),
    Assigment(AssigmentExpression),
    BlockStatement { body: Vec<Expression> },
}
