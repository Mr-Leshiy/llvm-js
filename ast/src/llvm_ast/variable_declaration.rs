use super::{VariableAssigment, VariableValue};
use compiler::{self, Compile, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl Compile for VariableDeclaration {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        let variable = self.0;
        let var = match variable.value {
            VariableValue::FloatNumber(value) => {
                Variable::new_number(compiler, value, &variable.name)
            }
            VariableValue::String(value) => Variable::new_string(compiler, &value, &variable.name),
            VariableValue::Identifier(name) => {
                let variable = cur_function.get_variable(name.clone())?;
                Variable::new_variable(compiler, cur_function, &name, &variable)
            }
        };
        cur_function.insert_variable(variable.name, var)
    }
}
