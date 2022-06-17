use super::Expression;

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Expression>,
}
