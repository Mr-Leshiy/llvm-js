use super::{expression::Expression, FunctionDeclaration};
use crate::compiler::{self, Compile, Compiler, ModuleUnit};

pub struct Program {
    pub functions: Vec<FunctionDeclaration>,
    pub body: Vec<Expression>,
}

impl<'ctx> Compile<'ctx> for Program {
    fn compile(
        self,
        _compiler: &'ctx mut Compiler,
        _module: &ModuleUnit<'ctx>,
    ) -> Result<(), compiler::Error> {
        for _func in self.functions {}

        for _expr in self.body {
            // expr.compile(compiler, module)?;
        }
        Ok(())
    }
}
