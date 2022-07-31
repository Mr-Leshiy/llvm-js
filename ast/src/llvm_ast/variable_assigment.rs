use super::{Identifier, VariableValue};
use compiler::{self, Compile, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub name: Identifier,
    pub value: VariableValue,
}

impl Compile<Identifier> for VariableAssigment {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let variable1 = cur_function.get_variable(self.name)?;
        match self.value {
            VariableValue::Boolean(boolean) => {
                variable1.assign_boolean(compiler, boolean);
                Ok(())
            }
            VariableValue::FloatNumber(value) => {
                variable1.assign_number(compiler, value);
                Ok(())
            }
            VariableValue::String(value) => {
                variable1.assign_string(compiler, &value);
                Ok(())
            }
            VariableValue::Identifier(name) => {
                let variable2 = cur_function.get_variable(name)?;
                variable1.assign_variable(compiler, cur_function, &variable2);
                Ok(())
            }
            VariableValue::LogicalExpression(logical) => {
                let variable2 = logical.compile(compiler, cur_function)?;
                variable1.assign_variable(compiler, cur_function, &variable2);
                Ok(())
            }
        }
    }
}
