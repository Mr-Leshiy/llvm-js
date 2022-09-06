use self::{
    abort::AbortFn, assert::AssertFn, assert_eq::AssertEqFn, printf::PrintFn, strcmp::StrcmpFn,
    strlen::StrlenFn,
};
use crate::{Compiler, Error};

pub mod abort;
pub mod assert;
pub mod assert_eq;
pub mod printf;
pub mod strcmp;
pub mod strlen;

pub trait PredefineFunctionName {
    const NAME: &'static str;
}

pub struct PredefineFunctions<'ctx> {
    printf: Option<PrintFn<'ctx>>,
    assert: Option<AssertFn>,
    assert_eq: Option<AssertEqFn>,
    abort: Option<AbortFn<'ctx>>,
    strcmp: Option<StrcmpFn<'ctx>>,
    strlen: Option<StrlenFn<'ctx>>,
}

impl<'ctx> Default for PredefineFunctions<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ctx> PredefineFunctions<'ctx> {
    pub(crate) fn new() -> Self {
        Self {
            printf: None,
            assert: None,
            assert_eq: None,
            abort: None,
            strcmp: None,
            strlen: None,
        }
    }

    pub(crate) fn declare<Iter, T>(
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
        let mut strlen = None;
        for function_name in predefined_functions {
            match function_name.as_str() {
                PrintFn::NAME => printf = Some(PrintFn::declare(compiler)),
                AssertFn::NAME => assert = Some(AssertFn::declare()),
                AssertEqFn::NAME => assert_eq = Some(AssertEqFn::declare()),
                AbortFn::NAME => abort = Some(AbortFn::declare(compiler)),
                StrcmpFn::NAME => strcmp = Some(StrcmpFn::declare(compiler)),
                StrlenFn::NAME => strlen = Some(StrlenFn::declare(compiler)),
                _ => return Err(Error::UndeclaredFunction(function_name)),
            }
        }
        Ok(Self {
            printf,
            assert,
            assert_eq,
            abort,
            strcmp,
            strlen,
        })
    }

    fn get_fn<T, FnType: PredefineFunctionName>(
        func: Option<&FnType>,
    ) -> Result<&FnType, Error<T>> {
        func.ok_or_else(|| Error::UndeclaredFunction(FnType::NAME.to_string()))
    }

    pub fn get_print<T>(&self) -> Result<&PrintFn<'ctx>, Error<T>> {
        Self::get_fn(self.printf.as_ref())
    }

    pub fn get_assert<T>(&self) -> Result<&AssertFn, Error<T>> {
        Self::get_fn(self.assert.as_ref())
    }

    pub fn get_assert_eq<T>(&self) -> Result<&AssertEqFn, Error<T>> {
        Self::get_fn(self.assert_eq.as_ref())
    }

    pub fn get_abort<T>(&self) -> Result<&AbortFn<'ctx>, Error<T>> {
        Self::get_fn(self.abort.as_ref())
    }

    pub fn get_strcmp<T>(&self) -> Result<&StrcmpFn<'ctx>, Error<T>> {
        Self::get_fn(self.strcmp.as_ref())
    }

    pub fn get_strlen<T>(&self) -> Result<&StrlenFn<'ctx>, Error<T>> {
        Self::get_fn(self.strlen.as_ref())
    }
}
