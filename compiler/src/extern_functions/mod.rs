use self::{assert::AssertFn, printf::PrintFn};
use crate::{Compiler, Error};

pub mod assert;
pub mod printf;

pub trait ExternFunction<'ctx> {
    fn declare(compiler: &Compiler<'ctx>) -> Self;
}

pub trait ExternFunctionName<'ctx>: ExternFunction<'ctx> + Clone {
    const NAME: &'static str;
}

pub struct ExternFunctions<'ctx> {
    printf: Option<PrintFn<'ctx>>,
    assert: Option<AssertFn<'ctx>>,
}

impl<'ctx> Default for ExternFunctions<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ctx> ExternFunctions<'ctx> {
    pub fn new() -> Self {
        Self {
            printf: None,
            assert: None,
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
        for function_name in predefined_functions {
            match function_name.as_str() {
                PrintFn::NAME => printf = Some(PrintFn::declare(compiler)),
                AssertFn::NAME => assert = Some(AssertFn::declare(compiler)),
                _ => return Err(Error::UndefinedFunction(function_name)),
            }
        }
        Ok(Self { printf, assert })
    }

    pub fn get_print(&self) -> Result<&PrintFn<'ctx>, Error> {
        self.printf
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(PrintFn::NAME.to_string()))
    }

    pub fn get_assert(&self) -> Result<&AssertFn<'ctx>, Error> {
        self.assert
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(AssertFn::NAME.to_string()))
    }
}
