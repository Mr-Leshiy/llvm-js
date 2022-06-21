use crate::{ast::Identifier, map::Map};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{FunctionValue, PointerValue},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(Identifier),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(Identifier),
    // #[error("Function with this identifier {0} already declared")]
    // AlreadyDeclaredFunction(Identifier),
    #[error("Invalid compiled module, {0}")]
    InvalidModule(String),
    #[error("Cannot write module, {0}")]
    CannotWriteModule(#[from] std::io::Error),
}

pub trait Compile<'ctx> {
    fn compile(self, compiler: &mut Compiler<'ctx>, module: &Module<'ctx>) -> Result<(), Error>;
}

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,

    pub variables: Map<Identifier, PointerValue<'ctx>>,
    pub functions: Map<Identifier, FunctionValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            builder: context.create_builder(),

            variables: Map::new(),
            functions: Map::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModuleUnit;

    #[test]
    fn compile_program_from_file_test() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();

        let module = ModuleUnit::new("module_1".to_string(), file).unwrap();
        let context = Context::create();
        let mut compiler = Compiler::new(&context);
        let mut writer = Vec::new();

        module.compile_to(&mut compiler, &mut writer).unwrap();
    }
}
