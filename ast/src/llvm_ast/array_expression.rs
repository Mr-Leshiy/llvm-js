use super::{Identifier, VariableExpression};
use compiler::{self, Compiler, Function, Variable};

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayExpression {
    pub values: Vec<VariableExpression>,
}

impl ArrayExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let mut array = Vec::new();
        for el in self.values {
            array.push(el.compile(compiler, cur_function)?);
        }
        let res = Variable::new_array(compiler)?;
        Ok(res)
    }
}
