use super::Expression;

pub enum BlockStatement {
    BlockStatement(Box<BlockStatement>),
    Expression(Expression),
}
