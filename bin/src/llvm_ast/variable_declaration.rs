use super::{VariableAssigment, VariableValue};
use compiler::{self, Compile, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl Compile for VariableDeclaration {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        let variable = self.0;
        let var = match variable.value {
            VariableValue::FloatNumber(value) => {
                Variable::new_number(compiler, value, &variable.name)
            }
            VariableValue::String(value) => Variable::new_string(compiler, &value, &variable.name),
            VariableValue::Identifier(name) => {
                let variable = compiler.get_variable(name.clone(), cur_function)?;
                Variable::new_variable(compiler, cur_function, &name, &variable)
            }
        };
        compiler.insert_variable(variable.name, var)
    }
}
