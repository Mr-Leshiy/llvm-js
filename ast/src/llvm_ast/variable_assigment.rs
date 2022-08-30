use super::{Identifier, VariableExpression};
use compiler::{self, Compile, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub name: Identifier,
    pub value: VariableExpression,
}

impl Compile<Identifier> for VariableAssigment {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let variable1 = cur_function.get_variable(self.name)?;
        let variable = self.value.compile(compiler, cur_function)?;
        variable1.assign_variable(compiler, cur_function, &variable);
        Ok(())
    }
}
