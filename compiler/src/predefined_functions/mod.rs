use self::{
    abort::AbortFn,
    assert::AssertFn,
    assert_eq::AssertEqFn,
    variable::{AllocateFn, PrintFn, SetBooleanFn, SetNumberFn, SetStringFn, SetVariableFn},
};
use crate::{Compiler, Error};

pub mod abort;
pub mod assert;
pub mod assert_eq;
pub mod variable;

pub trait PredefineFunctionName {
    const NAME: &'static str;
}

pub struct PredefineFunctions<'ctx> {
    assert: Option<AssertFn>,
    assert_eq: Option<AssertEqFn>,
    abort: Option<AbortFn<'ctx>>,
    // variable functions
    allocate: Option<AllocateFn<'ctx>>,
    set_number: Option<SetNumberFn<'ctx>>,
    set_boolean: Option<SetBooleanFn<'ctx>>,
    set_string: Option<SetStringFn<'ctx>>,
    set_variable: Option<SetVariableFn<'ctx>>,
    printf: Option<PrintFn<'ctx>>,
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
            // variable functions
            allocate: None,
            set_number: None,
            set_boolean: None,
            set_string: None,
            set_variable: None,
        }
    }

    pub(crate) fn declare<T>(compiler: &mut Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let assert = Some(AssertFn::declare());
        let assert_eq = Some(AssertEqFn::declare());
        let abort = Some(AbortFn::declare(compiler));
        // variable functions
        let allocate = Some(AllocateFn::declare(compiler));
        let set_number = Some(SetNumberFn::declare(compiler));
        let set_boolean = Some(SetBooleanFn::declare(compiler));
        let set_string = Some(SetStringFn::declare(compiler));
        let set_variable = Some(SetVariableFn::declare(compiler));
        let printf = Some(PrintFn::declare(compiler));

        Ok(Self {
            assert,
            assert_eq,
            abort,
            allocate,
            set_number,
            set_boolean,
            set_string,
            set_variable,
            printf,
        })
    }

    fn get_fn<T, FnType: PredefineFunctionName>(
        func: Option<&FnType>,
    ) -> Result<&FnType, Error<T>> {
        func.ok_or_else(|| Error::UndeclaredFunction(FnType::NAME.to_string()))
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

    // variable functions
    pub fn get_allocate<T>(&self) -> Result<&AllocateFn<'ctx>, Error<T>> {
        Self::get_fn(self.allocate.as_ref())
    }

    pub fn get_set_number<T>(&self) -> Result<&SetNumberFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_number.as_ref())
    }

    pub fn get_set_boolean<T>(&self) -> Result<&SetBooleanFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_boolean.as_ref())
    }

    pub fn get_set_string<T>(&self) -> Result<&SetStringFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_string.as_ref())
    }

    pub fn get_set_variable<T>(&self) -> Result<&SetVariableFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_variable.as_ref())
    }

    pub fn get_print<T>(&self) -> Result<&PrintFn<'ctx>, Error<T>> {
        Self::get_fn(self.printf.as_ref())
    }
}
