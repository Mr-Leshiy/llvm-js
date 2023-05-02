use super::{Expression, VariableExpression};
use crate::{Compiler, CompilerError};
use compiler::if_else::generate_if_else;

#[derive(Clone, Debug, PartialEq)]
pub struct IfElseStatement {
    pub condition: VariableExpression,
    pub if_clause: Vec<Expression>,
    pub else_clause: Vec<Expression>,
}

impl IfElseStatement {
    pub fn compile(self, compiler: &mut Compiler) -> Result<bool, CompilerError> {
        let condition = self.condition.compile_get_variable(compiler)?;

        generate_if_else(compiler, &condition, self.if_clause, self.else_clause)
    }
}
