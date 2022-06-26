use super::{VariableAssigment, VariableDeclaration};
use crate::compiler::{self, Compile, Compiler};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
}

impl Compile for Expression {
    fn compile<'ctx>(self, compiler: &mut Compiler<'ctx>) -> Result<(), compiler::Error> {
        match self {
            Self::VariableDeclaration(variable_declaration) => {
                variable_declaration.compile(compiler)
            }
            Self::VariableAssigment(variable_assigment) => variable_assigment.compile(compiler),
        }
    }
}
