use super::{VariableAssigment, VariableValue};
use compiler::{self, Compile, Compiler, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration(pub VariableAssigment);

impl Compile for VariableDeclaration {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let variable = self.0;
        let var = match variable.value {
            VariableValue::FloatNumber(value) => {
                Variable::new_number(compiler, value, &variable.name)
            }
            VariableValue::String(value) => Variable::new_string(compiler, &value, &variable.name),
            VariableValue::Identifier(name) => {
                let variable = compiler
                    .variables
                    .get(&name)
                    .ok_or_else(|| compiler::Error::UndefinedVariable(name.clone()))?
                    .clone();
                Variable::new_variable(compiler, &name, &variable)
            }
        };
        match compiler.variables.insert(variable.name.clone(), var) {
            None => Ok(()),
            Some(_) => Err(compiler::Error::AlreadyDeclaredVariable(variable.name)),
        }
    }
}
