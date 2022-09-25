use super::{Expression, Identifier, VariableExpression};
use compiler::{Compiler, Function};

#[derive(Clone, Debug, PartialEq)]
pub struct IfElseStatement {
    pub condition: VariableExpression,
    pub if_clause: Vec<Expression>,
    pub else_clause: Option<Vec<Expression>>,
}

impl IfElseStatement {
    pub fn compile<'ctx>(
        self,
        _compiler: &mut Compiler<'ctx, Identifier>,
        _cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        // todo!("implement");
        Ok(())
    }
}
