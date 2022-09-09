use super::{Compiler, PredefineFunctionName};
use crate::{Error, Function, Variable};

#[derive(Clone)]
pub struct AssertFn;

impl PredefineFunctionName for AssertFn {
    const NAME: &'static str = "assert";
}

impl AssertFn {
    pub(super) fn declare() -> Self {
        Self
    }

    pub fn call<'ctx, T>(
        &self,
        _compiler: &Compiler<'ctx, T>,
        _cur_function: &Function<'ctx, T>,
        _arg: Variable<'ctx>,
    ) -> Result<(), Error<T>> {
        todo!("implement")
    }
}
