use compiler::{self, Compile, Compiler, Function};

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
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        let variable1 = compiler.get_variable(self.name, cur_function)?;
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
                let variable2 = compiler.get_variable(name, cur_function)?;
                variable1.assign_variable(compiler, cur_function, &variable2);
                Ok(())
            }
        }
    }
}
