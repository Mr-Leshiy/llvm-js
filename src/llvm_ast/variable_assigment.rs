use crate::compiler::{self, Compile, Compiler};

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
        match compiler.variables.get(&self.name).cloned() {
            Some(pointer) => match self.value {
                VariableValue::FloatNumber(value) => {
                    let value = compiler.context.f64_type().const_float(value);
                    compiler.builder.build_store(pointer, value);
                    Ok(())
                }
                VariableValue::String(value) => {
                    let value = compiler.context.const_string(value.as_bytes(), false);
                    compiler.builder.build_store(pointer, value);
                    Ok(())
                }
                VariableValue::Identifier(name) => {
                    let value = compiler
                        .variables
                        .get(&name)
                        .ok_or_else(|| compiler::Error::UndefinedVariable(name.clone()))?;
                    let value = compiler.builder.build_load(*value, "");
                    compiler.builder.build_store(pointer, value);
                    Ok(())
                }
            },
            None => Err(compiler::Error::UndefinedVariable(self.name)),
        }
    }
}
