use super::{Expression, Identifier, VariableExpression};
use compiler::{generate_while_loop, Compiler, Function};

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
        let condition = |compiler: &mut Compiler<'ctx, Identifier>,
                         cur_function: &mut Function<'ctx, Identifier>| {
            self.condition.compile(compiler, cur_function)
        };

        generate_while_loop(compiler, condition, cur_function, self.body)
    }
}
