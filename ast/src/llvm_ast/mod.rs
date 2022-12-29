use crate::Error;
pub use array_expression::ArrayExpression;
pub use binary_expression::{BinaryExpType, BinaryExpression};
use compiler::{Compiler, Context, MainFunction};
pub use deallocate_expression::DeallocateExpression;
pub use do_while_loop::DoWhileLoop;
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
pub use if_else_statement::IfElseStatement;
pub use member_expression::{MemberExpression, Property};
pub use object_expression::ObjectExpression;
pub use return_statement::ReturnStatement;
use std::io::Write;
pub use unary_expression::{UnaryExpType, UnaryExpression};
pub use variable_assigment::VariableAssigment;
pub use variable_declaration::VariableDeclaration;
pub use variable_expression::VariableExpression;
pub use variable_value::VariableValue;
pub use while_loop::WhileLoop;

mod array_expression;
mod binary_expression;
mod deallocate_expression;
mod do_while_loop;
mod expression;
mod function_call;
mod function_declaration;
mod identifier;
mod if_else_statement;
mod member_expression;
mod object_expression;
mod return_statement;
mod unary_expression;
mod variable_assigment;
mod variable_declaration;
mod variable_expression;
mod variable_value;
mod while_loop;

pub struct Module {
    name: String,
    functions: Vec<FunctionDeclaration>,
    body: Vec<Expression>,
}

impl Module {
    pub fn new(name: String, functions: Vec<FunctionDeclaration>, body: Vec<Expression>) -> Self {
        Self {
            name,
            functions,
            body,
        }
    }

    pub fn compile_to<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        let context = Context::new();
        let compiler = &mut Compiler::new(&context, self.name.as_str());

        compiler.declare_extern_functions();

        for func in self.functions {
            func.compile(compiler)?;
        }
        // define main function
        let mut main = MainFunction::new(compiler);
        main.generate_body(compiler, self.body)?;

        compiler.write_result_into(writer)?;
        Ok(())
    }
}
