use inkwell::{context::Context, values::AnyValue};
use thiserror::Error;

mod literal;

#[derive(Debug, Error)]
pub enum Error {}

pub trait Compile {
    fn compile<'ctx>(
        &self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Box<dyn AnyValue<'ctx> + 'ctx>, Error>;
}

pub struct Compiler<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self { context }
    }
}

#[cfg(test)]
mod tests {}
