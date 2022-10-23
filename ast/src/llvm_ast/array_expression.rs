use super::{Identifier, VariableExpression};
use compiler::{self, Compiler, Function};

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayExpression {
    pub values: Vec<VariableExpression>,
}

impl ArrayExpression {
    pub fn compile<'ctx>(
        self,
        _compiler: &mut Compiler<'ctx, Identifier>,
        _cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        Ok(())
    }
}
