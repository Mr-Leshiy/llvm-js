use super::{Compiler, PredefineFunctionName};
use crate::{Error, Function, Variable};

#[derive(Clone)]
pub struct AssertEqFn;

impl PredefineFunctionName for AssertEqFn {
    const NAME: &'static str = "assert_eq";
}

impl AssertEqFn {
    pub(super) fn declare() -> Self {
        Self
    }

    pub fn assert_eq<'ctx, T>(
        &self,
        _compiler: &Compiler<'ctx, T>,
        _cur_function: &Function<'ctx, T>,
        _arg1: Variable<'ctx>,
        _arg2: Variable<'ctx>,
    ) -> Result<(), Error<T>> {
        todo!("implement")
    }
}
