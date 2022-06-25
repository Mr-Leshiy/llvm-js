use crate::llvm_ast::VariableName;
use inkwell::{context::Context, module::Module, values::PointerValue};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(VariableName),
    #[error("Invalid compiled module, {0}")]
    InvalidModule(String),
}

pub trait Compile<'ctx> {
    fn compile(self, compiler: &'ctx mut Compiler, module: &ModuleUnit<'ctx>) -> Result<(), Error>;
}

pub struct Compiler<'ctx> {
    pub context: Context,

    pub variables: HashMap<VariableName, PointerValue<'ctx>>,
}

#[derive(Clone)]
pub struct ModuleUnit<'ctx> {
    pub module: Module<'ctx>,
}

impl<'ctx> ModuleUnit<'ctx> {
    pub fn verify(&self) -> Result<(), Error> {
        self.module
            .verify()
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        Ok(())
    }
}

impl<'ctx> Compiler<'ctx> {
    pub fn new() -> Self {
        Self {
            context: Context::create(),
            variables: HashMap::new(),
        }
    }

    pub fn create_module(&self, module_name: &str) -> ModuleUnit {
        ModuleUnit {
            module: self.context.create_module(module_name),
        }
    }
}
