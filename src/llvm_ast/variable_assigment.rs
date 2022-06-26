use crate::compiler::{self, Compile, Compiler};
use inkwell::values::AnyValue;

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
    fn compile<'ctx>(self, compiler: &mut Compiler<'ctx>) -> Result<(), compiler::Error> {
        match compiler.variables.get(&self.name).cloned() {
            Some(pointer) => match self.value {
                VariableValue::FloatNumber(value) => {
                    println!("here 2.1");
                    let value = compiler.context.f64_type().const_float(value);
                    compiler.builder.build_store(pointer, value);
                    Ok(())
                }
                VariableValue::String(value) => {
                    println!("here 2.2");
                    let value = compiler.context.const_string(value.as_bytes(), false);
                    compiler.builder.build_store(pointer, value);
                    Ok(())
                }
                VariableValue::Identifier(name) => {
                    let value = compiler
                        .variables
                        .get(&name)
                        .ok_or_else(|| compiler::Error::UndefinedVariable(name.clone()))?;
                    println!("here 2.3 : {0}", value.print_to_string());
                    let value = compiler.builder.build_load(*value, "");
                    println!("here 2.3");
                    compiler.builder.build_store(pointer, value);
                    Ok(())
                }
            },
            None => Err(compiler::Error::UndefinedVariable(self.name)),
        }
    }
}
