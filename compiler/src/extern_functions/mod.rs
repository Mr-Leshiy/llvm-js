use self::printf::PrintFn;
use crate::{Compiler, Error};

pub mod printf;

pub trait ExternFunction<'ctx> {
    fn declare(compiler: &Compiler<'ctx>) -> Self;
}

pub trait ExternFunctionName<'ctx>: ExternFunction<'ctx> + Clone {
    const NAME: &'static str;
}

pub struct ExternFunctions<'ctx> {
    printf: Option<PrintFn<'ctx>>,
}

impl<'ctx> Default for ExternFunctions<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ctx> ExternFunctions<'ctx> {
    pub fn new() -> Self {
        Self { printf: None }
    }

    pub fn declare<Iter>(
        compiler: &mut Compiler<'ctx>,
        predefined_functions: Iter,
    ) -> Result<Self, Error>
    where
        Iter: Iterator<Item = String>,
    {
        let mut printf = None;
        for function_name in predefined_functions {
            match function_name.as_str() {
                PrintFn::NAME => printf = Some(PrintFn::declare(compiler)),
                _ => return Err(Error::UndefinedFunction(function_name)),
            }
        }
        Ok(Self { printf })
    }

    pub fn get_print(&self) -> Result<&PrintFn<'ctx>, Error> {
        self.printf
            .as_ref()
            .ok_or_else(|| Error::UndeclaredFunction(PrintFn::NAME.to_string()))
    }
}
