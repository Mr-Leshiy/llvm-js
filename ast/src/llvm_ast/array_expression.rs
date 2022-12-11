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
        let res = Variable::new_array(compiler, true)?;
        for (i, el) in self.values.into_iter().enumerate() {
            let value = el.compile(compiler, cur_function)?;
            res.add_property_by_str(compiler, i.to_string().as_str(), &value)?;
            if value.is_tmp() {
                value.deallocate(compiler)?;
            }
        }
        Ok(res)
    }
}
