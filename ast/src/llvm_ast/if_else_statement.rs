use super::{Expression, Identifier, VariableExpression};
use compiler::{generate_if_else, Compiler, Function};

#[derive(Clone, Debug, PartialEq)]
pub struct IfElseStatement {
    pub condition: VariableExpression,
    pub if_clause: Vec<Expression>,
    pub else_clause: Vec<Expression>,
}

impl IfElseStatement {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
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
