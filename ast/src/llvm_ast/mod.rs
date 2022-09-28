pub use binary_expression::{BinaryExpType, BinaryExpression};
use compiler::{self, Compiler, Context};
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
pub use if_else_statement::IfElseStatement;
pub use program::Program;
pub use return_statement::ReturnStatement;
use std::io::Write;
pub use unary_expression::{UnaryExpType, UnaryExpression};
pub use variable_assigment::VariableAssigment;
pub use variable_declaration::VariableDeclaration;
pub use variable_expression::VariableExpression;
pub use variable_value::VariableValue;
pub use while_loop::WhileLoop;

mod binary_expression;
mod expression;
mod function_call;
mod function_declaration;
mod identifier;
mod if_else_statement;
mod program;
mod return_statement;
mod unary_expression;
mod variable_assigment;
mod variable_declaration;
mod variable_expression;
mod variable_value;
mod while_loop;

pub struct Module {
    pub name: String,
    pub program: Program,
}

impl Module {
    pub fn compile_to<W: Write>(self, writer: &mut W) -> Result<(), compiler::Error<Identifier>> {
        let context = Context::new();
        let mut compiler = Compiler::new(&context, self.name.as_str());

        compiler.declare_extern_functions();

        self.program.compile(&mut compiler)?;
        compiler.write_result_into(writer)
    }
}
