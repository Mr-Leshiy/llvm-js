use super::{Identifier, VariableExpression};
use compiler::{Compile, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub ret: VariableExpression,
}

impl Compile<Identifier> for ReturnStatement {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        self.ret.compile(compiler, cur_function)?;
        Ok(())
    }
}
