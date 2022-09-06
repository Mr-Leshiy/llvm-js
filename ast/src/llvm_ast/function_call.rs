use super::{Identifier, VariableExpression};
use compiler::{
    self,
    predefined_functions::{
        assert::AssertFn, assert_eq::AssertEqFn, printf::PrintFn, PredefineFunctionName,
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
                let pritnf = compiler.predefined_functions().get_print()?;
                pritnf.call(
                    compiler,
                    cur_function,
                    args.into_iter()
                        .next()
                        .ok_or(compiler::Error::NotEnoughArguments)?,
                )
            }
            AssertFn::NAME => {
                let assert = compiler.predefined_functions().get_assert()?;
                assert.call(
                    compiler,
                    cur_function,
                    args.into_iter()
                        .next()
                        .ok_or(compiler::Error::NotEnoughArguments)?,
                )
            }
            AssertEqFn::NAME => {
                let assert_eq = compiler.predefined_functions().get_assert_eq()?;
                let mut iter = args.into_iter();
                assert_eq.assert_eq(
                    compiler,
                    cur_function,
                    iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                    iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                )
            }
            _ => {
                let function = compiler.get_function(self.name)?;
                function.generate_call(compiler, args)
            }
        }
    }
}
