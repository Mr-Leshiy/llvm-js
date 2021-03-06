pub use context::Context;
pub use function::Function;
use predefined_functions::PredefineFunctions;
use std::{collections::HashMap, hash::Hash, io::Write};
use thiserror::Error;
pub use variable::Variable;

mod context;
mod function;
pub mod logical_operations;
pub mod predefined_functions;
mod variable;

#[derive(Debug, Error)]
pub enum Error<T> {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(T),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(T),
    #[error("Undefined function identifier {0}")]
    UndefinedFunction(T),
    #[error("Function with this identifier {0} already declared")]
    AlreadyDeclaredFunction(T),
    #[error("Not enough arguments")]
    NotEnoughArguments,
    #[error("Invalid compiled module, {0}")]
    InvalidModule(String),
    #[error("Cannot write module, {0}")]
    CannotWriteModule(String),
    #[error("Undeclared function: {0}")]
    UndeclaredFunction(String),
}

pub trait Compile<T> {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, T>,
        cur_function: &mut Function<'ctx, T>,
    ) -> Result<(), Error<T>>;
}

pub struct Compiler<'ctx, T> {
    context: &'ctx Context,
    module: inkwell::module::Module<'ctx>,
    builder: inkwell::builder::Builder<'ctx>,

    functions: HashMap<T, Function<'ctx, T>>,
    predefined_functions: PredefineFunctions<'ctx>,
}

impl<'ctx, T> Compiler<'ctx, T> {
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
    ) -> Result<(), Error<T>>
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

impl<'ctx, T> Compiler<'ctx, T>
where
    T: Clone + Hash + PartialEq + Eq,
{
    pub fn insert_function(
        &mut self,
        name: T,
        function: Function<'ctx, T>,
    ) -> Result<(), Error<T>> {
        match self.functions.insert(name.clone(), function) {
            None => Ok(()),
            Some(_) => Err(Error::AlreadyDeclaredFunction(name)),
        }
    }

    pub fn get_function(&self, name: T) -> Result<Function<'ctx, T>, Error<T>> {
        self.functions
            .get(&name)
            .cloned()
            .ok_or(Error::UndefinedFunction(name))
    }

    pub fn write_result_into<W: Write>(&self, writer: &mut W) -> Result<(), Error<T>> {
        self.verify()?;
        writer
            .write(self.module.print_to_string().to_bytes())
            .map_err(|e| Error::CannotWriteModule(e.to_string()))?;
        Ok(())
    }

    pub fn verify(&self) -> Result<(), Error<T>> {
        self.module
            .verify()
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        Ok(())
    }
}
