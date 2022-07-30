pub use context::Context;
pub use function::Function;
use predefined_functions::PredefineFunctions;
use std::{collections::HashMap, io::Write};
use thiserror::Error;
pub use variable::{Variable, VariableValue};

mod context;
mod function;
pub mod logical_operations;
pub mod predefined_functions;
mod variable;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(String),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(String),
    #[error("Undefined function identifier {0}")]
    UndefinedFunction(String),
    #[error("Function with this identifier {0} already declared")]
    AlreadyDeclaredFunction(String),
    #[error("Not enough arguments")]
    NotEnoughArguments,
    #[error("Invalid compiled module, {0}")]
    InvalidModule(String),
    #[error("Cannot write module, {0}")]
    CannotWriteModule(String),
    #[error("Undeclared function: {0}")]
    UndeclaredFunction(String),
}

pub trait Compile {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), Error>;
}

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: inkwell::module::Module<'ctx>,
    builder: inkwell::builder::Builder<'ctx>,

    functions: HashMap<String, Function<'ctx>>,
    predefined_functions: PredefineFunctions<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            functions: HashMap::new(),
            predefined_functions: PredefineFunctions::new(),
        }
    }

    pub fn declare_extern_functions<Iter>(
        &mut self,
        predefined_functions: Iter,
    ) -> Result<(), Error>
    where
        Iter: Iterator<Item = String>,
    {
        self.predefined_functions = PredefineFunctions::declare(self, predefined_functions)?;
        Ok(())
    }

    pub fn predefined_functions(&self) -> &PredefineFunctions<'ctx> {
        &self.predefined_functions
    }
}

impl<'ctx> Compiler<'ctx> {
    pub fn insert_function(&mut self, name: String, function: Function<'ctx>) -> Result<(), Error> {
        match self.functions.insert(name.clone(), function) {
            None => Ok(()),
            Some(_) => Err(Error::AlreadyDeclaredFunction(name)),
        }
    }

    pub fn get_function(&self, name: String) -> Result<Function<'ctx>, Error> {
        self.functions
            .get(&name)
            .cloned()
            .ok_or(Error::UndefinedFunction(name))
    }

    pub fn write_result_into<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        self.verify()?;
        writer
            .write(self.module.print_to_string().to_bytes())
            .map_err(|e| Error::CannotWriteModule(e.to_string()))?;
        Ok(())
    }

    pub fn verify(&self) -> Result<(), Error> {
        self.module
            .verify()
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        Ok(())
    }
}
