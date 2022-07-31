use super::Identifier;
use compiler::{self, Compile, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Boolean(bool),
    FloatNumber(f64),
    String(String),
    Identifier(Identifier),
}

impl From<VariableValue> for compiler::VariableValue {
    fn from(val: VariableValue) -> Self {
        match val {
            VariableValue::Boolean(boolean) => compiler::VariableValue::Boolean(boolean),
            VariableValue::FloatNumber(number) => compiler::VariableValue::FloatNumber(number),
            VariableValue::String(string) => compiler::VariableValue::String(string),
            VariableValue::Identifier(ident) => compiler::VariableValue::Identifier(ident.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub name: Identifier,
    pub value: VariableValue,
}

impl Compile for VariableAssigment {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        let variable1 = cur_function.get_variable(self.name.into())?;
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
                let variable2 = cur_function.get_variable(name.into())?;
                variable1.assign_variable(compiler, cur_function, &variable2);
                Ok(())
            }
        }
    }
}
