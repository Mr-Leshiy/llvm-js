use super::{VariableAssigment, VariableValue, Identifier};
use compiler::{self, Compile, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl Compile<Identifier> for VariableDeclaration {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>>
    {
        let variable = self.0;
        let var = match variable.value {
            VariableValue::Boolean(boolean) => Variable::new_boolean(compiler, boolean),
            VariableValue::FloatNumber(value) => Variable::new_number(compiler, value),
            VariableValue::String(value) => Variable::new_string(compiler, &value),
            VariableValue::Identifier(name) => {
                let variable1 = cur_function.get_variable(name)?;
                Variable::new_variable(compiler, cur_function, &variable1)
            }
            VariableValue::LogicalExpression(logical) => {
                let variable1 = logical.compile(compiler, cur_function)?;
                Variable::new_variable(compiler, cur_function, &variable1)
            }
        };
        cur_function.insert_variable(variable.name.into(), var)
    }
}
