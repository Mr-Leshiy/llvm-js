use super::VariableExpression;
use crate::{Compiler, CompilerError};
use compiler::Variable;

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
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        match self.exp_type {
            UnaryExpType::Not => {
                let var = self.exp.compile_get_variable(compiler)?;
                let logical_not_fn = compiler.predefined_functions().logical_not();
                let ret = logical_not_fn.call(compiler, &var);
                if var.is_tmp() {
                    var.deallocate(compiler);
                }
                Ok(ret)
            }
        }
    }
}
