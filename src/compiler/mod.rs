use crate::ast::Identifier;
use inkwell::{
    builder::Builder,
    context::Context,
    values::{AnyValue, PointerValue},
};
use std::collections::HashMap;
use thiserror::Error;

mod literal;
mod variable_declaration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable, {0}")]
    UndefinedVariable(Identifier),
}

pub trait CompileResult {
    fn to_string(&self) -> String;
}

impl<'ctx> CompileResult for PointerValue<'ctx> {
    fn to_string(&self) -> String {
        self.print_to_string().to_string()
    }
}

pub trait Compile<'ctx> {
    type Output: CompileResult;
    fn compile(&self, compiler: &mut Compiler<'ctx>) -> Result<Self::Output, Error>;
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

#[cfg(test)]
mod tests {}
