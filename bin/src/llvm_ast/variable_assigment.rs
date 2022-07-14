use compiler::{self, Compile, Compiler};

pub type VariableName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    FloatNumber(f64),
    String(String),
    Identifier(VariableName),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssigment {
    pub name: VariableName,
    pub value: VariableValue,
}

impl Compile for VariableAssigment {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let variable1 = compiler.get_variable(self.name)?;
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
                let variable2 = compiler.get_variable(name)?;
                variable1.assign_variable(compiler, &variable2);
                Ok(())
            }
        }
    }
}
