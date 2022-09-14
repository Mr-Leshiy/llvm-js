use super::{Identifier, VariableExpression};
use compiler::{
    self,
    predefined_functions::{
        assertions::{AssertEqFn, AssertFn},
        variable::PrintFn,
        PredefineFunctionName,
    },
    Compile, Compiler, Function,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<VariableExpression>,
}

impl Compile<Identifier> for FunctionCall {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let mut args = Vec::new();
        for arg in self.args.into_iter() {
            args.push(arg.compile(compiler, cur_function)?);
        }
        match String::from(self.name.clone()).as_str() {
            PrintFn::NAME => {
                let mut iter = args.into_iter();
                let pritnf = compiler.predefined_functions().get_print()?;
                pritnf.call(
                    compiler,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                );
                Ok(())
            }
            AssertFn::NAME => {
                let mut iter = args.into_iter();
                let assert_fn = compiler.predefined_functions().get_assert()?;
                assert_fn.call(
                    compiler,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                );
                Ok(())
            }
            AssertEqFn::NAME => {
                let mut iter = args.into_iter();
                let assert_eq_fn = compiler.predefined_functions().get_assert_eq()?;
                assert_eq_fn.call(
                    compiler,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                );
                Ok(())
            }
            _ => {
                let function = compiler.get_function(self.name)?;
                function.call(compiler, args)
            }
        }
    }
}
