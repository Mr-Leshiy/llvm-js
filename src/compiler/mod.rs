use crate::{
    ast::{Identifier, ModuleUnit},
    map::Map,
};
use inkwell::{builder::Builder, context::Context, module::Module, values::PointerValue};
use std::io::Write;
use thiserror::Error;

mod assigment_expression;
mod expression;
mod literal;
mod program;
mod variable_declaration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(Identifier),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(Identifier),
    #[error("Invalid compiled module, {0}")]
    InvalidModule(String),
    #[error("Cannot write module, {0}")]
    CannotWriteModule(#[from] std::io::Error),
}

pub trait CompileResult {
    fn to_string(&self) -> String;
}

pub trait Compile<'ctx> {
    type Output: CompileResult;
    fn compile(
        self,
        compiler: &mut Compiler<'ctx>,
        module: &Module<'ctx>,
    ) -> Result<Self::Output, Error>;
}

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,

    variables: Map<Identifier, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            builder: context.create_builder(),

            variables: Map::new(),
        }
    }
}

impl ModuleUnit {
    pub fn compile_to<W: Write>(
        self,
        compiler: &mut Compiler<'_>,
        writer: &mut W,
    ) -> Result<(), Error> {
        let module = compiler.context.create_module(self.name.as_str());
        self.program.compile(compiler, &module)?;
        module
            .verify()
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        writer.write_all(module.print_to_string().to_bytes())?;
        Ok(())
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
        let mut writer = Vec::new();

        module.compile_to(&mut compiler, &mut writer).unwrap();
    }
}
