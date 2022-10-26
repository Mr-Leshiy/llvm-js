use super::{Identifier, VariableExpression};
use compiler::{self, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub value: Option<VariableExpression>,
}

impl VariableDeclaration {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let var = Variable::new_undefined(compiler)?;

        if let Some(value) = self.value {
            let value = value.compile(compiler, cur_function)?;
            var.assign_variable(compiler, &value)?;
        }

        cur_function.insert_variable(self.name, var)
    }
}
