use super::{VariableAssigment, VariableDeclaration};
use crate::compiler::{self, Compile, Compiler, ModuleUnit};

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
}

impl<'ctx> Compile<'ctx> for Expression {
    fn compile(
        self,
        compiler: &'ctx mut Compiler,
        module: &ModuleUnit<'ctx>,
    ) -> Result<(), compiler::Error> {
        match self {
            Self::VariableDeclaration(variable_declaration) => {
                variable_declaration.compile(compiler, module)
            }
            Self::VariableAssigment(variable_assigment) => {
                variable_assigment.compile(compiler, module)
            }
        }
    }
}
