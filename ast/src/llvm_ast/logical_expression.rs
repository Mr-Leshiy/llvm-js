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
            Self::Not(value) => logical_not(compiler, cur_function, value.into())?,
            Self::And { left, right } => {
                logical_and(compiler, cur_function, left.into(), right.into())?
            }
            Self::Or { left, right } => {
                logical_or(compiler, cur_function, left.into(), right.into())?
            }
        };
        Ok(())
    }
}
