use super::{FunctionCall, Identifier, ReturnStatement, VariableAssigment, VariableDeclaration};
use compiler::{self, Compile, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
    FunctionCall(FunctionCall),
    ReturnStatement(ReturnStatement),
}

impl Compile<Identifier> for Expression {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        match self {
            Self::VariableDeclaration(variable_declaration) => {
                variable_declaration.compile(compiler, cur_function)
            }
            Self::VariableAssigment(variable_assigment) => {
                variable_assigment.compile(compiler, cur_function)
            }
            Self::FunctionCall(function_call) => function_call.compile(compiler, cur_function),
            Self::ReturnStatement(return_statement) => {
                return_statement.compile(compiler, cur_function)
            }
        }
    }
}
