use super::{Identifier, VariableValue};
use compiler::{
    logical_operations::{logical_and, logical_not, logical_or},
    Compiler, Function, Variable,
};

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalExpression {
    Not(VariableValue),
    And {
        left: VariableValue,
        right: VariableValue,
    },
    Or {
        left: VariableValue,
        right: VariableValue,
    },
}

impl LogicalExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        match self {
            Self::Not(value) => {
                let value = value.compile(compiler, cur_function)?;
                logical_not(compiler, cur_function, value)
            }
            Self::And { left, right } => {
                let left = left.compile(compiler, cur_function)?;
                let right = right.compile(compiler, cur_function)?;
                logical_and(compiler, cur_function, left, right)
            }
            Self::Or { left, right } => {
                let left = left.compile(compiler, cur_function)?;
                let right = right.compile(compiler, cur_function)?;
                logical_or(compiler, cur_function, left, right)
            }
        }
    }
}
