use super::{Identifier, VariableExpression};
use compiler::{Compiler, Function, Variable};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectExpression {
    pub properties: HashMap<Identifier, VariableExpression>,
}

impl ObjectExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let res = Variable::new_object(compiler, true)?;
        for (key, value) in self.properties {
            let value = value.compile(compiler, cur_function)?;
            res.add_property_by_str(compiler, &String::from(key), &value)?;
        }
        Ok(res)
    }
}
