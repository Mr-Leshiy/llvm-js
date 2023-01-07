use super::{
    DeallocateExpression, DoWhileLoop, FunctionCall, Identifier, IfElseStatement, ReturnStatement,
    VariableAssigment, VariableDeclaration, WhileLoop,
};
use crate::{Compiler, CompilerError, Function};
use compiler::Compile;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
    DeallocateExpression(DeallocateExpression),
    FunctionCall(FunctionCall),
    ReturnStatement(ReturnStatement),
    IfElseStatement(IfElseStatement),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
}

impl Compile<Identifier> for Expression {
    type Output = bool;

    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<Self::Output, CompilerError> {
        match self {
            Self::VariableDeclaration(variable_declaration) => {
                variable_declaration.compile(compiler, cur_function)?;
                Ok(false)
            }
            Self::VariableAssigment(variable_assigment) => {
                variable_assigment.compile(compiler, cur_function)?;
                Ok(false)
            }
            Self::DeallocateExpression(deallocate_expression) => {
                deallocate_expression.compile(compiler, cur_function)?;
                Ok(false)
            }
            Self::FunctionCall(function_call) => {
                let var = function_call.compile(compiler, cur_function)?;
                var.deallocate(compiler)?;
                Ok(false)
            }
            Self::ReturnStatement(return_statement) => {
                return_statement.compile(compiler, cur_function)?;
                Ok(true)
            }
            Self::IfElseStatement(if_else_statement) => {
                if_else_statement.compile(compiler, cur_function)
            }
            Self::WhileLoop(while_loop) => {
                while_loop.compile(compiler, cur_function)?;
                Ok(false)
            }
            Self::DoWhileLoop(do_while_loop) => {
                do_while_loop.compile(compiler, cur_function)?;
                Ok(false)
            }
        }
    }
}
