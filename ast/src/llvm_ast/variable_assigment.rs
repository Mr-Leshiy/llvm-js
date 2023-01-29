use super::{MemberExpression, VariableExpression};
use crate::{Compiler, CompilerError};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub left: MemberExpression,
    pub right: VariableExpression,
}

impl VariableAssigment {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let var1 = self.left.compile(compiler, true)?;
        let var = self.right.compile(compiler)?;
        var1.assign_variable(compiler, &var);
        if var.is_tmp() {
            var.deallocate(compiler);
        }
        Ok(())
    }
}
