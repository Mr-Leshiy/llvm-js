use super::VariableExpression;
use crate::{Compiler, CompilerError};
use compiler::Variable;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayExpression {
    pub values: Vec<VariableExpression>,
}

impl ArrayExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let res = Variable::new_array(compiler, true);
        for (i, el) in self.values.into_iter().enumerate() {
            let value = el.compile(compiler)?;
            res.add_property_by_number(
                compiler,
                u32::try_from(i).expect("number overflow").into(),
                &value,
            );
        }
        Ok(res)
    }
}
