use super::{Identifier, VariableExpression};
use compiler::{Compiler, Function};

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
        let ret = self.ret.compile(compiler, cur_function)?;
        cur_function.return_value(compiler, ret);
        Ok(())
    }
}
