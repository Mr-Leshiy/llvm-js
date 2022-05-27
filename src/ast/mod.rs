pub use assigment_expression::AssigmentExpression;
pub use identifier::Identifier;
pub use literal::Literal;
pub use right_assignment_value::RightAssigmentValue;
pub use variable_declaration::VariableDeclaration;

mod assigment_expression;
mod identifier;
mod literal;
mod right_assignment_value;
mod variable_declaration;

pub trait Expr {}

/// Program
pub struct Program {
    pub body: Vec<Box<dyn Expr>>,
}
