use super::{MemberExpression, VariableExpression};
use crate::{Compiler, CompilerError};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub left: MemberExpression,
    pub right: Option<VariableExpression>,
}

impl VariableAssigment {
    pub fn compile<'ctx>(self, compiler: &mut Compiler<'ctx>) -> Result<(), CompilerError> {
        let var1 = self.left.compile(compiler, true)?;
        if let Some(value) = self.right {
            let var = value.compile(compiler)?;
            var1.assign_variable(compiler, &var)?;
            if var.is_tmp() {
                var.deallocate(compiler)?;
            }
        }
        Ok(())
    }
}
