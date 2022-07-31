use super::VariableValue;
use compiler::{
    logical_operations::{logical_and, logical_not, logical_or},
    Compile, Compiler, Function,
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

impl Compile for LogicalExpression {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        match self {
            Self::Not(value) => {
                let value = value.compile(compiler, cur_function)?;
                logical_not(compiler, cur_function, value)?
            }
            Self::And { left, right } => {
                let left = left.compile(compiler, cur_function)?;
                let right = right.compile(compiler, cur_function)?;
                logical_and(compiler, cur_function, left, right)?
            }
            Self::Or { left, right } => {
                let left = left.compile(compiler, cur_function)?;
                let right = right.compile(compiler, cur_function)?;
                logical_or(compiler, cur_function, left, right)?
            }
        };
        Ok(())
    }
}
