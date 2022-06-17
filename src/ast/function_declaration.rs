use super::{BlockStatement, Identifier};

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: BlockStatement,
}
