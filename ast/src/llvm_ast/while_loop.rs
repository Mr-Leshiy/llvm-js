use super::{Expression, VariableExpression};
use crate::{Compiler, CompilerError, Function};
use compiler::loops::generate_while_loop;

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: VariableExpression,
    pub body: Vec<Expression>,
}

impl WhileLoop {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), CompilerError> {
        let condition = |compiler: &mut Compiler<'ctx>, cur_function: &mut Function<'ctx>| {
            self.condition.compile(compiler, cur_function)
        };

        generate_while_loop(compiler, condition, cur_function, self.body)
    }
}
