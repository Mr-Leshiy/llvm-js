use self::{
    abort::AbortFn, assert::AssertFn, assert_eq::AssertEqFn, printf::PrintFn, strcmp::StrcmpFn,
};
use crate::{Compiler, Error};

pub mod abort;
pub mod assert;
pub mod assert_eq;
pub mod printf;
pub mod strcmp;

pub trait PredefineFunctionName {
    const NAME: &'static str;
}

pub struct PredefineFunctions<'ctx> {
    printf: Option<PrintFn<'ctx>>,
    assert: Option<AssertFn>,
    assert_eq: Option<AssertEqFn>,
    abort: Option<AbortFn<'ctx>>,
    strcmp: Option<StrcmpFn<'ctx>>,
}

impl<'ctx> Default for PredefineFunctions<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ctx> PredefineFunctions<'ctx> {
    pub fn new() -> Self {
        Self {
            printf: None,
            assert: None,
            assert_eq: None,
            abort: None,
            strcmp: None,
        }
    }

    pub fn declare<Iter, T>(
        compiler: &mut Compiler<'ctx, T>,
        predefined_functions: Iter,
    ) -> Result<Self, Error<T>>
    where
        Iter: Iterator<Item = String>,
    {
        let mut printf = None;
        let mut assert = None;
        let mut assert_eq = None;
        let mut abort = None;
        let mut strcmp = None;
        for function_name in predefined_functions {
            match function_name.as_str() {
                PrintFn::NAME => printf = Some(PrintFn::declare(compiler)),
                AssertFn::NAME => assert = Some(AssertFn::declare()),
                AssertEqFn::NAME => assert_eq = Some(AssertEqFn::declare()),
                AbortFn::NAME => abort = Some(AbortFn::declare(compiler)),
                StrcmpFn::NAME => strcmp = Some(StrcmpFn::declare(compiler)),
                _ => return Err(Error::UndeclaredFunction(function_name)),
            }
        }
        Ok(Self {
            printf,
            assert,
            assert_eq,
            abort,
            strcmp,
        })
    }

    pub fn get_print<T>(&self) -> Result<&PrintFn<'ctx>, Error<T>> {
        self.printf
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(PrintFn::NAME.to_string()))
    }

    pub fn get_assert<T>(&self) -> Result<&AssertFn, Error<T>> {
        self.assert
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(AssertFn::NAME.to_string()))
    }

    pub fn get_assert_eq<T>(&self) -> Result<&AssertEqFn, Error<T>> {
        self.assert_eq
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(AssertEqFn::NAME.to_string()))
    }

    pub fn get_abort<T>(&self) -> Result<&AbortFn<'ctx>, Error<T>> {
        self.abort
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(AbortFn::NAME.to_string()))
    }

    pub fn get_strcmp<T>(&self) -> Result<&StrcmpFn<'ctx>, Error<T>> {
        self.strcmp
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(StrcmpFn::NAME.to_string()))
    }
}
