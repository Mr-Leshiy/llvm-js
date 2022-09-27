use super::{Identifier, VariableExpression};
use compiler::{Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub exp: VariableExpression,
    pub exp_type: UnaryExpType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
                let logical_not_fn = compiler.predefined_functions()?.logical_not();
                Ok(logical_not_fn.call(compiler, &var))
            }
        }
    }
}
