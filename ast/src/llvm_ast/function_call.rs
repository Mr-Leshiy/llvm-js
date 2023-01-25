use super::{Identifier, VariableExpression};
use crate::{Compiler, CompilerError};
use compiler::{
    self,
    predefined_functions::{
        test::{AssertEqFn, AssertFn, PrintFn},
        PredefineFunctionName,
    },
    Variable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<VariableExpression>,
}

impl FunctionCall {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let mut args = Vec::new();
        for arg in self.args {
            let value = arg.compile(compiler)?;
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
            _ => {
                let function = compiler.get_function(self.name)?;
                function.call(compiler, &args)
            }
        };
        // deallocate arguments
        for arg in args {
            arg.deallocate(compiler)?;
        }
        res
    }
}
