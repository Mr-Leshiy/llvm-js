use super::{VariableAssigment, VariableValue};
use crate::compiler::{self, Compile, Compiler};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl Compile for VariableDeclaration {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let variable = self.0;
        let pointer = match variable.value {
            VariableValue::FloatNumber(value) => {
                let value = compiler.context.f64_type().const_float(value);
                let pointer = compiler
                    .builder
                    .build_alloca(value.get_type(), variable.name.as_str());
                compiler.builder.build_store(pointer, value);
                compiler.get_printf()?.print(compiler, pointer)?;
                pointer
            }
            VariableValue::String(value) => {
                let value = compiler.context.const_string(value.as_bytes(), false);
                let pointer = compiler
                    .builder
                    .build_alloca(value.get_type(), variable.name.as_str());

                compiler.builder.build_store(pointer, value);
                compiler.get_printf()?.print(compiler, pointer)?;
                pointer
            }
            VariableValue::Identifier(name) => {
                let value = compiler
                    .variables
                    .get(&name)
                    .ok_or_else(|| compiler::Error::UndefinedVariable(name.clone()))?;

                let value = compiler.builder.build_load(*value, name.as_str());
                let pointer = compiler
                    .builder
                    .build_alloca(value.get_type(), variable.name.as_str());
                compiler.builder.build_store(pointer, value);
                pointer
            }
        };
        match compiler.variables.insert(variable.name.clone(), pointer) {
            None => Ok(()),
            Some(_) => Err(compiler::Error::AlreadyDeclaredVariable(variable.name)),
        }
    }
}
