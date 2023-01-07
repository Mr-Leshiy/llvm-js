use super::{Expression, VariableExpression};
use crate::{CompilerError, Compiler, Function};
use compiler::{loops::generate_do_while_loop};

#[derive(Clone, Debug, PartialEq)]
pub struct DoWhileLoop {
    pub condition: VariableExpression,
    pub body: Vec<Expression>,
}

impl DoWhileLoop {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), CompilerError> {
        let condition = |compiler: &mut Compiler<'ctx>,
                         cur_function: &mut Function<'ctx>| {
            self.condition.compile(compiler, cur_function)
        };

        generate_do_while_loop(compiler, condition, cur_function, self.body)
    }
}
