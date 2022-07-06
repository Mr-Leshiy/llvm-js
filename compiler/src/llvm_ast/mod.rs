use crate::compiler::{self, Compile, Compiler};
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::{FunctionDeclaration, FunctionName};
use inkwell::context::Context;
pub use program::Program;
use std::io::Write;
pub use variable_assigment::{VariableAssigment, VariableName, VariableValue};
pub use variable_declaration::VariableDeclaration;

mod expression;
mod function_call;
mod function_declaration;
mod program;
mod variable_assigment;
mod variable_declaration;

pub struct Module {
    pub name: String,
    pub program: Program,
}

impl Module {
    pub fn compile_to<W: Write>(self, writer: &mut W) -> Result<(), compiler::Error> {
        let context = Context::create();
        let mut compiler = Compiler::new(&context, self.name.as_str());

        compiler.declare_prinf();

        self.program.compile(&mut compiler)?;
        compiler.verify()?;
        writer
            .write(compiler.module.print_to_string().to_bytes())
            .map_err(|e| compiler::Error::CannotWriteModule(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::js_ast;

    #[test]
    fn compile_module_from_file_test() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let module = js_ast::Module::new("".to_string(), file).unwrap();
        let module = module.precompile().unwrap();
        let mut out = Vec::new();
        module.compile_to(&mut out).unwrap();
    }
}
