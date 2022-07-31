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
            VariableValue::Boolean(boolean) => {
                Variable::new_boolean(compiler, boolean, &String::from(variable.name.clone()))
            }
            VariableValue::FloatNumber(value) => {
                Variable::new_number(compiler, value, &String::from(variable.name.clone()))
            }
            VariableValue::String(value) => {
                Variable::new_string(compiler, &value, &String::from(variable.name.clone()))
            }
            VariableValue::Identifier(name) => {
                let variable1 = cur_function.get_variable(name.clone().into())?;
                Variable::new_variable(
                    compiler,
                    cur_function,
                    &String::from(variable.name.clone()),
                    &variable1,
                )
            }
            VariableValue::LogicalExpression(logical) => {
                let variable1 = logical.compile(compiler, cur_function)?;
                Variable::new_variable(
                    compiler,
                    cur_function,
                    &String::from(variable.name.clone()),
                    &variable1,
                )
            }
        };
        cur_function.insert_variable(variable.name.into(), var)
    }
}
