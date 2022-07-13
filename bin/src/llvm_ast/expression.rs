use super::{FunctionCall, VariableAssigment, VariableDeclaration};
use compiler::{self, Compile, Compiler};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
    FunctionCall(FunctionCall),
}

impl Compile for Expression {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        match self {
            Self::VariableDeclaration(variable_declaration) => {
                variable_declaration.compile(compiler)
            }
            Self::VariableAssigment(variable_assigment) => variable_assigment.compile(compiler),
            Self::FunctionCall(function_call) => function_call.compile(compiler),
        }
    }
}
