use super::Identifier;
use compiler::{Compiler, Function};

#[derive(Clone, Debug, PartialEq)]
pub struct DeallocateExpression {
    pub name: Identifier,
}

impl DeallocateExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let deallocate_fn = compiler.predefined_functions()?.deallocate();
        let var = cur_function.get_variable(self.name)?;
        deallocate_fn.call(compiler, &var);
        Ok(())
    }
}
