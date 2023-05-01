#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

pub use context::Context;
pub use function::Function;
use inkwell::{builder::Builder, module::Module, types::PointerType, AddressSpace};
pub use main_function::MainFunction;
use predefined_functions::PredefineFunctions;
use std::{collections::HashMap, hash::Hash, path::Path};
pub use variable::Variable;

mod context;
mod function;
pub mod if_else;
pub mod loops;
mod main_function;
pub mod predefined_functions;
mod variable;

#[derive(thiserror::Error, Debug, PartialEq)]
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
    type Output;

    fn compile(self, compiler: &mut Compiler<T>) -> Result<Self::Output, Error<T>>;
}

struct InkwellContext<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variable_type: PointerType<'ctx>,
}

pub struct Compiler<'ctx, T> {
    inkwell_context: InkwellContext<'ctx>,

    functions: HashMap<T, Function<'ctx, T>>,
    predefined_functions: PredefineFunctions<'ctx>,

    cur_function: Option<Function<'ctx, T>>,
}

impl<'ctx, T> Compiler<'ctx, T> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let inkwell_context = InkwellContext {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            variable_type: context
                .opaque_struct_type(Variable::TYPE_NAME)
                .ptr_type(AddressSpace::from(0)),
        };
        let predefined_functions = PredefineFunctions::declare(&inkwell_context);
        Self {
            inkwell_context,
            functions: HashMap::new(),
            predefined_functions,
            cur_function: None,
        }
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

    pub fn insert_variable(&mut self, name: T, variable: Variable<'ctx>) -> Result<(), Error<T>> {
        self.cur_function
            .as_mut()
            .unwrap()
            .insert_variable(name, variable)
    }

    pub fn get_variable(&self, name: T) -> Result<Variable<'ctx>, Error<T>> {
        let cur_function = self.cur_function.as_ref().unwrap();
        cur_function.get_variable(self, name)
    }

    pub fn generate_llvm_ir(&self, path: &Path) -> Result<(), Error<T>> {
        self.verify()?;
        // self.inkwell_context.module.write_bitcode_to_path(path);
        self.inkwell_context
            .module
            .print_to_file(path)
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        Ok(())
    }

    pub fn verify(&self) -> Result<(), Error<T>> {
        self.inkwell_context
            .module
            .verify()
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        Ok(())
    }
}
