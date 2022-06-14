pub use assigment_expression::AssigmentExpression;
pub use expression::Expression;
pub use identifier::Identifier;
pub use literal::Literal;
pub use program::Program;
pub use right_assignment_value::RightAssigmentValue;

mod assigment_expression;
mod expression;
mod identifier;
mod literal;
mod program;
mod right_assignment_value;

/// Module
pub struct ModuleUnit {
    pub name: String,
    pub program: Program,
}
