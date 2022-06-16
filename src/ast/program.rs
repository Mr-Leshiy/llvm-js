use super::Expression;

/// Program
#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Expression>,
}
