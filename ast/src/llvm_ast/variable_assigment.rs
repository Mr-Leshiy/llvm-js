use super::{Identifier, MemberExpression, VariableExpression};
use crate::CompilerError;
use compiler::{Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub left: MemberExpression,
    pub right: Option<VariableExpression>,
}

impl VariableAssigment {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), CompilerError> {
        let var1 = self.left.compile(compiler, cur_function, true)?;
        match self.right {
            Some(value) => {
                let var = value.compile(compiler, cur_function)?;
                var1.assign_variable(compiler, &var)
            }
            None => Ok(()),
        }
    }
}
