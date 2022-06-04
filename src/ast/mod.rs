pub use assigment_expression::AssigmentExpression;
pub use identifier::Identifier;
pub use literal::Literal;
pub use right_assignment_value::RightAssigmentValue;
use std::fmt::Debug;
pub use variable_declaration::VariableDeclaration;

mod assigment_expression;
mod identifier;
mod literal;
mod right_assignment_value;
mod variable_declaration;

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    AssigmentExpression(AssigmentExpression),
}

/// Module
pub struct Module {
    pub name: String,
    pub program: Program,
}

/// Program
pub struct Program {
    pub body: Vec<Expression>,
}
