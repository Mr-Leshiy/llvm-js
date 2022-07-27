use self::{abort::AbortFn, assert::AssertFn, assert_eq::AssertEqFn, printf::PrintFn};
use crate::{Compiler, Error};

pub mod abort;
pub mod assert;
pub mod assert_eq;
pub mod printf;

pub trait PredefineFunctionName {
    const NAME: &'static str;
}

pub struct PredefineFunctions<'ctx> {
    printf: Option<PrintFn<'ctx>>,
    assert: Option<AssertFn>,
    assert_eq: Option<AssertEqFn>,
    abort: Option<AbortFn<'ctx>>,
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
        }
    }

    pub fn declare<Iter>(
        compiler: &mut Compiler<'ctx>,
        predefined_functions: Iter,
    ) -> Result<Self, Error>
    where
        Iter: Iterator<Item = String>,
    {
        let mut printf = None;
        let mut assert = None;
        let mut assert_eq = None;
        let mut abort = None;
        for function_name in predefined_functions {
            match function_name.as_str() {
                PrintFn::NAME => printf = Some(PrintFn::declare(compiler)),
                AssertFn::NAME => assert = Some(AssertFn::declare()),
                AssertEqFn::NAME => assert_eq = Some(AssertEqFn::declare()),
                AbortFn::NAME => abort = Some(AbortFn::declare(compiler)),
                _ => return Err(Error::UndefinedFunction(function_name)),
            }
        }
        Ok(Self {
            printf,
            assert,
            assert_eq,
            abort,
        })
    }

    pub fn get_print(&self) -> Result<&PrintFn<'ctx>, Error> {
        self.printf
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(PrintFn::NAME.to_string()))
    }

    pub fn get_assert(&self) -> Result<&AssertFn, Error> {
        self.assert
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(AssertFn::NAME.to_string()))
    }

    pub fn get_assert_eq(&self) -> Result<&AssertEqFn, Error> {
        self.assert_eq
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(AssertEqFn::NAME.to_string()))
    }

    pub fn get_abort(&self) -> Result<&AbortFn<'ctx>, Error> {
        self.abort
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(AbortFn::NAME.to_string()))
    }
}
