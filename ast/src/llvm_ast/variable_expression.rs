use super::{BinaryExpression, FunctionCall, UnaryExpression, VariableValue};
use crate::{Compiler, CompilerError, Function};
use compiler::Variable;

#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    VariableValue(VariableValue),
    UnaryExpression(Box<UnaryExpression>),
    BinaryExpression(Box<BinaryExpression>),
    FunctionCall(FunctionCall),
}

impl VariableExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        match self {
            Self::VariableValue(value) => value.compile(compiler, cur_function),
            Self::UnaryExpression(expr) => expr.compile(compiler, cur_function),
            Self::BinaryExpression(expr) => expr.compile(compiler, cur_function),
            Self::FunctionCall(function_call) => function_call.compile(compiler, cur_function),
        }
    }
}
