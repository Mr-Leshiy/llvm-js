use super::Identifier;
use crate::{Compiler, CompilerError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeallocateExpression {
    pub name: Identifier,
}

impl DeallocateExpression {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        println!("deallocate, {self:?}");
        let var = compiler.get_variable(self.name)?;
        var.deallocate(compiler);
        Ok(())
    }
}
