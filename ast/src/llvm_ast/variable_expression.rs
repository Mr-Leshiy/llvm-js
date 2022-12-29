use super::{BinaryExpression, FunctionCall, Identifier, UnaryExpression, VariableValue};
use compiler::{Compiler, Function, Variable};

/// VariableExpression
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
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        match self {
            Self::VariableValue(value) => value.compile(compiler, cur_function),
            Self::UnaryExpression(expr) => expr.compile(compiler, cur_function),
            Self::BinaryExpression(expr) => expr.compile(compiler, cur_function),
            Self::FunctionCall(function_call) => function_call.compile(compiler, cur_function),
        }
    }
}
