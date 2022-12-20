use super::Identifier;
use compiler::{Compiler, Function};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeallocateExpression {
    pub name: Identifier,
}

impl DeallocateExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let var = cur_function.get_variable(self.name)?;
        var.deallocate(compiler)?;
        Ok(())
    }
}
