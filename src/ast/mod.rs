pub use assigment_expression::{AssigmentExpression, VariableDeclaration};
pub use block_statement::BlockStatement;
pub use expression::Expression;
pub use identifier::Identifier;
pub use literal::Literal;
pub use program::Program;
pub use right_assignment_value::RightAssigmentValue;

mod assigment_expression;
mod block_statement;
mod expression;
mod function_declaration;
mod identifier;
mod literal;
mod program;
mod right_assignment_value;

/// Module
pub struct ModuleUnit {
    pub name: String,
    pub program: Program,
}
