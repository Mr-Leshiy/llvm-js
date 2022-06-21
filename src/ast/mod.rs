use crate::compiler::{self, Compile, Compiler};
pub use assigment_expression::AssigmentExpression;
pub use block_statement::BlockStatement;
pub use expression::Expression;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
pub use literal::Literal;
pub use program::Program;
pub use right_assignment_value::RightAssigmentValue;
use std::io::Write;
pub use variable_declaration::VariableDeclaration;

mod assigment_expression;
mod block_statement;
mod expression;
mod function_declaration;
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

impl ModuleUnit {
    pub fn compile_to<W: Write>(
        self,
        compiler: &mut Compiler<'_>,
        writer: &mut W,
    ) -> Result<(), compiler::Error> {
        let module = compiler.context.create_module(self.name.as_str());
        self.program.compile(compiler, &module)?;
        module
            .verify()
            .map_err(|e| compiler::Error::InvalidModule(e.to_string()))?;
        writer.write_all(module.print_to_string().to_bytes())?;
        Ok(())
    }
}
