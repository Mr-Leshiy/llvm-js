use super::{Identifier, VariableExpression};
use crate::{Compiler, CompilerError};
use compiler::Variable;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub value: Option<VariableExpression>,
}

impl VariableDeclaration {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let var = Variable::new_undefined(compiler, false)?;

        if let Some(value) = self.value {
            let value = value.compile(compiler)?;
            var.assign_variable(compiler, &value)?;
            if value.is_tmp() {
                value.deallocate(compiler)?;
            }
        }

        compiler.insert_variable(self.name, var)
    }
}
