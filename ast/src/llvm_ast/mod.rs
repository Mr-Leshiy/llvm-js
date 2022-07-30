use compiler::{self, Compiler, Context};
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::{FunctionDeclaration, FunctionName};
pub use logical_expression::LogicalExpression;
pub use program::Program;
use std::io::Write;
pub use variable_assigment::{VariableAssigment, VariableName, VariableValue};
pub use variable_declaration::VariableDeclaration;

mod expression;
mod function_call;
mod function_declaration;
mod logical_expression;
mod program;
mod variable_assigment;
mod variable_declaration;

pub struct Module {
    pub name: String,
    pub program: Program,
}

impl Module {
    pub fn compile_to<W: Write, Iter>(
        self,
        writer: &mut W,
        extern_functions: Iter,
    ) -> Result<(), compiler::Error>
    where
        Iter: Iterator<Item = String>,
    {
        let context = Context::new();
        let mut compiler = Compiler::new(&context, self.name.as_str());

        compiler.declare_extern_functions(extern_functions)?;

        self.program.compile(&mut compiler)?;
        compiler.write_result_into(writer)
    }
}
