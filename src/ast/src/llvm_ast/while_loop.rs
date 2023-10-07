use super::{Expression, VariableExpression};
use crate::{Compiler, CompilerError};
use compiler::loops::generate_while_loop;

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: VariableExpression,
    pub body: Vec<Expression>,
}

impl WhileLoop {
    pub fn compile<'ctx>(self, compiler: &mut Compiler<'ctx>) -> Result<(), CompilerError> {
        let condition =
            |compiler: &mut Compiler<'ctx>| self.condition.compile_get_variable(compiler);

        generate_while_loop(compiler, condition, self.body)
    }
}
