use super::{Identifier, VariableExpression};
use compiler::{
    self,
    predefined_functions::{
        test::{AssertEqFn, AssertFn, GbVariablesCount, PrintFn},
        PredefineFunctionName,
    },
    Compiler, Function, Variable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<VariableExpression>,
}

impl FunctionCall {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let mut args = Vec::new();
        for arg in self.args.into_iter() {
            let value = arg.compile(compiler, cur_function)?;
            let arg = Variable::new_undefined(compiler, true)?;
            arg.assign_variable(compiler, &value)?;
            if value.is_tmp() {
                value.deallocate(compiler)?;
            }
            args.push(arg);
        }
        let res = match String::from(self.name.clone()).as_str() {
            PrintFn::NAME => {
                let mut iter = args.clone().into_iter();
                let pritn = compiler.predefined_functions()?.print();
                pritn.call(
                    compiler,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                );
                Ok(Variable::new_undefined(compiler, true)?)
            }
            AssertFn::NAME => {
                let mut iter = args.clone().into_iter();
                let assert_fn = compiler.predefined_functions()?.assert();
                assert_fn.call(
                    compiler,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                );
                Ok(Variable::new_undefined(compiler, true)?)
            }
            AssertEqFn::NAME => {
                let mut iter = args.clone().into_iter();
                let assert_eq_fn = compiler.predefined_functions()?.assert_eq();
                assert_eq_fn.call(
                    compiler,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                    &iter.next().ok_or(compiler::Error::NotEnoughArguments)?,
                );
                Ok(Variable::new_undefined(compiler, true)?)
            }
            GbVariablesCount::NAME => {
                let gb_variables_count = compiler.predefined_functions()?.gb_variables_count();
                Ok(gb_variables_count.call(compiler))
            }
            _ => {
                let function = compiler.get_function(self.name)?;
                function.call(compiler, args.clone())
            }
        };
        // deallocate arguments
        for arg in args {
            arg.deallocate(compiler)?;
        }
        res
    }
}
