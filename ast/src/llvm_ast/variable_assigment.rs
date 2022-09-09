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
        let var1 = cur_function.get_variable(self.name)?;
        let var = self.value.compile(compiler, cur_function)?;
        var1.assign_variable(compiler, &var)
    }
}
