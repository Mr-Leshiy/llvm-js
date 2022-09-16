use super::{Identifier, VariableAssigment};
use compiler::{self, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl VariableDeclaration {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let variable = self.0;

        match variable.value {
            Some(value) => {
                let var = value.compile(compiler, cur_function)?;
                cur_function.insert_variable(variable.name, var)
            }
            None => {
                let var = Variable::new_undefined(compiler)?;
                cur_function.insert_variable(variable.name, var)
            }
        }
    }
}
