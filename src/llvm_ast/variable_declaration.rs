use super::{VariableAssigment, VariableValue};
use crate::compiler::{self, Compile, Compiler, ModuleUnit};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl<'ctx> Compile<'ctx> for VariableDeclaration {
    fn compile(self, compiler: &mut Compiler, _: &ModuleUnit<'ctx>) -> Result<(), compiler::Error> {
        let builder = compiler.context.create_builder();
        let variable = self.0;
        match variable.value {
            VariableValue::FloatNumber(value) => {
                let value = compiler.context.f64_type().const_float(value);
                let pointer = builder.build_alloca(value.get_type(), variable.name.as_str());
                builder.build_store(pointer, value);
            }
            VariableValue::String(value) => {
                let value = compiler.context.const_string(value.as_bytes(), false);
                let pointer = builder.build_alloca(value.get_type(), variable.name.as_str());
                builder.build_store(pointer, value);
            }
            VariableValue::Identifier(name) => {
                let value = compiler
                    .variables
                    .get(&name)
                    .ok_or_else(|| compiler::Error::UndefinedVariable(name.clone()))?;

                let value = builder.build_load(*value, name.as_str());
                let pointer = builder.build_alloca(value.get_type(), variable.name.as_str());
                builder.build_store(pointer, value);
            }
        };
        Ok(())
    }
}
