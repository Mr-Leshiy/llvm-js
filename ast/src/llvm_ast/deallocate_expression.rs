use super::Identifier;
use crate::{CompilerError, Compiler, Function};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeallocateExpression {
    pub name: Identifier,
}

impl DeallocateExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), CompilerError> {
        let var = cur_function.get_variable(self.name)?;
        var.deallocate(compiler)?;
        Ok(())
    }
}
