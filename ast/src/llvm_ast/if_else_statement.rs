use super::{Expression, VariableExpression};
use crate::{CompilerError, Compiler, Function};
use compiler::{if_else::generate_if_else};

#[derive(Clone, Debug, PartialEq)]
pub struct IfElseStatement {
    pub condition: VariableExpression,
    pub if_clause: Vec<Expression>,
    pub else_clause: Vec<Expression>,
}

impl IfElseStatement {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<bool, CompilerError> {
        let condition = self.condition.compile(compiler, cur_function)?;

        generate_if_else(
            compiler,
            &condition,
            cur_function,
            self.if_clause,
            self.else_clause,
        )
    }
}
