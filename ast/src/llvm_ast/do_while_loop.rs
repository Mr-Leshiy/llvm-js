use super::{Expression, VariableExpression};
use crate::{Compiler, CompilerError};
use compiler::loops::generate_do_while_loop;

#[derive(Clone, Debug, PartialEq)]
pub struct DoWhileLoop {
    pub condition: VariableExpression,
    pub body: Vec<Expression>,
}

impl DoWhileLoop {
    pub fn compile<'ctx>(self, compiler: &mut Compiler<'ctx>) -> Result<(), CompilerError> {
        let condition = |compiler: &mut Compiler<'ctx>| self.condition.compile(compiler);

        generate_do_while_loop(compiler, condition, self.body)
    }
}
