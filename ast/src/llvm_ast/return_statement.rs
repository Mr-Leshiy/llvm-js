use super::{Identifier, VariableExpression};
use compiler::{Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub ret: VariableExpression,
}

impl ReturnStatement {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let value = self.ret.compile(compiler, cur_function)?;
        let ret = Variable::new_undefined(compiler, true)?;
        ret.assign_variable(compiler, &value)?;
        if value.is_tmp() {
            value.deallocate(compiler)?;
        }
        cur_function.return_value(compiler, ret);
        Ok(())
    }
}
