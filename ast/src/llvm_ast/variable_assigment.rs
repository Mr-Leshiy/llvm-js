use super::VariableExpression;
use crate::{Compiler, CompilerError};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub left: VariableExpression,
    pub right: VariableExpression,
}

impl VariableAssigment {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let left = self.left.compile(compiler)?;
        let right = self.right.compile(compiler)?;
        left.assign_variable(compiler, &right);
        if right.is_tmp() {
            right.deallocate(compiler);
        }
        if left.is_tmp() {
            left.deallocate(compiler);
        }
        Ok(())
    }
}
