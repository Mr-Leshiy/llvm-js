use super::Expression;
use crate::compiler::{self, Compile, Compiler, ModuleUnit};

pub type FunctionName = String;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: FunctionName,
    pub body: Vec<Expression>,
}

impl<'ctx> Compile<'ctx> for FunctionDeclaration {
    fn compile(
        self,
        compiler: &'ctx mut Compiler,
        module: &ModuleUnit<'ctx>,
    ) -> Result<(), compiler::Error> {
        let function_type = compiler.context.void_type().fn_type(&[], false);
        module
            .module
            .add_function(self.name.as_str(), function_type, None);

        Ok(())
    }
}
