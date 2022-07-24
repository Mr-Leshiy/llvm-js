use super::{FunctionName, VariableValue};
use compiler::{
    self,
    extern_functions::{assert::AssertFn, printf::PrintFn, ExternFunctionName},
    Compile, Compiler, Function,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: FunctionName,
    pub args: Vec<VariableValue>,
}

impl Compile for FunctionCall {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        let args: Vec<_> = self
            .args
            .into_iter()
            .map(|variable| variable.into())
            .collect();
        match self.name.as_str() {
            PrintFn::NAME => {
                let pritnf = compiler.extern_functions().get_print()?;
                pritnf.print(
                    compiler,
                    cur_function,
                    args.into_iter()
                        .next()
                        .ok_or(compiler::Error::NotEnoughArguments)?,
                )
            }
            AssertFn::NAME => {
                let assert = compiler.extern_functions().get_assert()?;
                assert.assert(
                    compiler,
                    cur_function,
                    args.into_iter()
                        .next()
                        .ok_or(compiler::Error::NotEnoughArguments)?,
                )
            }
            _ => {
                let function = compiler.get_function(self.name)?;
                function.generate_call(compiler, cur_function, args)
            }
        }
    }
}
