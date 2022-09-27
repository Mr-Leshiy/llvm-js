use super::{Expression, Identifier, VariableExpression};
use compiler::{Compiler, Function};

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: VariableExpression,
    pub body: Vec<Expression>,
}

impl WhileLoop {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let _condition = self.condition.compile(compiler, cur_function)?;
        // TODO implement

        Ok(())
    }
}
