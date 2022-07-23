use compiler::{self, Compile, Compiler, Function};

pub type VariableName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    FloatNumber(f64),
    String(String),
    Identifier(VariableName),
}

impl From<VariableValue> for compiler::VariableValue {
    fn from(val: VariableValue) -> Self {
        match val {
            VariableValue::FloatNumber(number) => compiler::VariableValue::FloatNumber(number),
            VariableValue::String(string) => compiler::VariableValue::String(string),
            VariableValue::Identifier(ident) => compiler::VariableValue::Identifier(ident),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub name: VariableName,
    pub value: VariableValue,
}

impl Compile for VariableAssigment {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        let variable1 = cur_function.get_variable(self.name)?;
        match self.value {
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
        }
    }
}
