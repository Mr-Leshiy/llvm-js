use super::VariableExpression;
use crate::{Compiler, CompilerError};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub left: VariableExpression,
    pub right: VariableExpression,
}

impl VariableAssigment {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let right = self.right.compile_get_variable(compiler)?;
        self.left.compile_update_variable(compiler, &right)?;
        if right.is_tmp() {
            right.deallocate(compiler);
        }
        Ok(())
    }
}
