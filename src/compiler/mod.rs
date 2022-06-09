use crate::ast::{Identifier, ModuleUnit};
use inkwell::{builder::Builder, context::Context, module::Module, values::PointerValue};
use std::collections::HashMap;
use thiserror::Error;

mod assigment_expression;
mod literal;
mod program;
mod variable_declaration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable, {0}")]
    UndefinedVariable(Identifier),
}

pub trait CompileResult {
    fn to_string(&self) -> String;
}

pub trait Compile<'ctx> {
    type Output: CompileResult;
    // TODO: consider to the change to the 'self' instead of '&self'
    fn compile(
        &self,
        compiler: &mut Compiler<'ctx>,
        module: &Module<'ctx>,
    ) -> Result<Self::Output, Error>;
}

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,

    variables: HashMap<Identifier, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            builder: context.create_builder(),

            variables: HashMap::new(),
        }
    }
}

impl ModuleUnit {
    pub fn compile_to(self, compiler: &mut Compiler<'_>) -> Result<(), Error> {
        let module = compiler.context.create_module(self.name.as_str());
        self.program.compile(compiler, &module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_program_from_file() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();

        let module = ModuleUnit::new("module_1".to_string(), file).unwrap();
        let context = Context::create();
        let mut compiler = Compiler::new(&context);

        module.compile_to(&mut compiler).unwrap();
    }
}
