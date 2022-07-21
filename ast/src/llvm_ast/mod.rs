use compiler::{self, Compiler, Context};
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::{FunctionDeclaration, FunctionName};
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

#[cfg(test)]
mod tests {
    use crate::js_ast;

    #[test]
    fn compile_module_from_file_test() {
        let file = std::fs::File::open("../test_scripts/basic.js").unwrap();
        let module = js_ast::Module::new("".to_string(), file).unwrap();
        let module = module.precompile(vec![].into_iter()).unwrap();
        let mut out = Vec::new();
        module.compile_to(&mut out, vec![].into_iter()).unwrap();
    }
}
