use super::{
    DeallocateExpression, DoWhileLoop, Identifier, IfElseStatement, ReturnStatement,
    VariableAssigment, VariableDeclaration, VariableExpression, VariableFunctionDeclaration,
    WhileLoop,
};
use crate::{Compiler, CompilerError};
use compiler::Compile;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    VariableFunctionDeclaration(VariableFunctionDeclaration),
    VariableAssigment(VariableAssigment),
    VariableExpression(VariableExpression),
    DeallocateExpression(DeallocateExpression),
    ReturnStatement(ReturnStatement),
    IfElseStatement(IfElseStatement),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
}

impl Compile<Identifier> for Expression {
    type Output = bool;

    fn compile(self, compiler: &mut Compiler) -> Result<Self::Output, CompilerError> {
        match self {
            Self::VariableDeclaration(variable_declaration) => {
                variable_declaration.compile(compiler)?;
                Ok(false)
            }
            Self::VariableFunctionDeclaration(variable_function_declaration) => {
                variable_function_declaration.compile(compiler)?;
                Ok(false)
            }
            Self::VariableAssigment(variable_assigment) => {
                variable_assigment.compile(compiler)?;
                Ok(false)
            }
            Self::VariableExpression(variable_expression) => {
                variable_expression.compile_get_variable(compiler)?;
                Ok(false)
            }
            Self::DeallocateExpression(deallocate_expression) => {
                deallocate_expression.compile(compiler)?;
                Ok(false)
            }
            Self::ReturnStatement(return_statement) => {
                return_statement.compile(compiler)?;
                Ok(true)
            }
            Self::IfElseStatement(if_else_statement) => if_else_statement.compile(compiler),
            Self::WhileLoop(while_loop) => {
                while_loop.compile(compiler)?;
                Ok(false)
            }
            Self::DoWhileLoop(do_while_loop) => {
                do_while_loop.compile(compiler)?;
                Ok(false)
            }
        }
    }
}
