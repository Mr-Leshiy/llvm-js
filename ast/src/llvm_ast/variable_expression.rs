use super::{Identifier, UnaryExpression, VariableValue};
use compiler::{self, Compiler, Function, Variable};

/// VariableExpression
#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    VariableValue(VariableValue),
    UnaryExpression(Box<UnaryExpression>),
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
        }
    }
}
