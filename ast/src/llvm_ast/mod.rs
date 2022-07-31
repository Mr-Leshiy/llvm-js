use compiler::{self, Compiler, Context};
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
pub use logical_expression::LogicalExpression;
pub use program::Program;
use std::io::Write;
pub use variable_assigment::VariableAssigment;
pub use variable_declaration::VariableDeclaration;
pub use variable_value::VariableValue;

mod expression;
mod function_call;
mod function_declaration;
mod identifier;
mod logical_expression;
mod program;
mod variable_assigment;
mod variable_declaration;
mod variable_value;

pub struct Module {
    pub name: String,
    pub program: Program,
}

impl Module {
    pub fn compile_to<W: Write, Iter>(
        self,
        writer: &mut W,
        extern_functions: Iter,
    ) -> Result<(), compiler::Error<Identifier>>
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
