use super::{Identifier, VariableExpression};
use crate::{Compiler, CompilerError};
use compiler::Variable;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectExpression {
    pub properties: HashMap<Identifier, VariableExpression>,
}

impl ObjectExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let res = Variable::new_object(compiler, true);
        for (key, value) in self.properties {
            let value = value.compile_get_variable(compiler)?;
            res.add_property_by_str(compiler, &String::from(key), &value, true);
        }
        Ok(res)
    }
}
