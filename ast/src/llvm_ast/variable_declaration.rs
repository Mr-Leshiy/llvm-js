use super::{Identifier, VariableAssigment};
use compiler::{self, Compile, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl Compile<Identifier> for VariableDeclaration {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let variable = self.0;

        let var = variable.value.compile(compiler, cur_function)?;
        cur_function.insert_variable(variable.name, var)
    }
}
