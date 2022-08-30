use super::{Identifier, VariableExpression};
use compiler::{logical_operations::logical_not, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub exp: VariableExpression,
    pub exp_type: UnaryExpType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExpType {
    Not,
}

impl UnaryExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        match self.exp_type {
            UnaryExpType::Not => {
                let var = self.exp.compile(compiler, cur_function)?;
                logical_not(compiler, cur_function, var)
            }
        }
    }
}
