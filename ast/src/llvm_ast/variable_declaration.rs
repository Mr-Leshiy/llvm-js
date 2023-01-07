use super::{Identifier, VariableExpression};
use crate::{Compiler, CompilerError, Function};
use compiler::Variable;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub value: Option<VariableExpression>,
}

impl VariableDeclaration {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), CompilerError> {
        let var = Variable::new_undefined(compiler, false)?;

        if let Some(value) = self.value {
            let value = value.compile(compiler, cur_function)?;
            var.assign_variable(compiler, &value)?;
            if value.is_tmp() {
                value.deallocate(compiler)?;
            }
        }

        cur_function.insert_variable(self.name, var)
    }
}
