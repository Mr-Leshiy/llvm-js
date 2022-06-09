pub use assigment_expression::AssigmentExpression;
pub use identifier::Identifier;
pub use literal::Literal;
pub use program::{Expression, Program};
pub use right_assignment_value::RightAssigmentValue;
pub use variable_declaration::VariableDeclaration;

mod assigment_expression;
mod identifier;
mod literal;
mod program;
mod right_assignment_value;
mod variable_declaration;

/// Module
pub struct ModuleUnit {
    pub name: String,
    pub program: Program,
}
