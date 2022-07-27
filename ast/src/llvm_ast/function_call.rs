use super::{FunctionName, VariableValue};
use compiler::{
    self,
    predefined_functions::{
        abort::AbortFn, assert::AssertFn, printf::PrintFn, PredefineFunctionName,
    },
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
        match String::from(self.name.clone()).as_str() {
            PrintFn::NAME => {
                let pritnf = compiler.predefined_functions().get_print()?;
                pritnf.print(
                    compiler,
                    cur_function,
                    args.into_iter()
                        .next()
                        .ok_or(compiler::Error::NotEnoughArguments)?,
                )
            }
            AssertFn::NAME => {
                let assert = compiler.predefined_functions().get_assert()?;
                let abort = compiler.predefined_functions().get_abort()?;
                assert.assert(
                    compiler,
                    cur_function,
                    abort,
                    args.into_iter()
                        .next()
                        .ok_or(compiler::Error::NotEnoughArguments)?,
                )
            }
            AbortFn::NAME => {
                let abort = compiler.predefined_functions().get_abort()?;
                abort.abort(compiler);
                Ok(())
            }
            _ => {
                let function = compiler.get_function(self.name.into())?;
                function.generate_call(compiler, cur_function, args)
            }
        }
    }
}
