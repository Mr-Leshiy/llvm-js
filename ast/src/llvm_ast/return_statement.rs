use super::VariableExpression;
use crate::{Compiler, CompilerError, Function};
use compiler::Variable;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub ret: VariableExpression,
}

impl ReturnStatement {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let value = self.ret.compile(compiler)?;
        let ret = Variable::new_undefined(compiler, true)?;
        ret.assign_variable(compiler, &value)?;
        value.deallocate(compiler)?;
        Function::return_value(compiler, &ret);
        Ok(())
    }
}
