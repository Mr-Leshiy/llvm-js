use super::Expression;
use crate::compiler::{self, Compile, Compiler};

pub type FunctionName = String;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: FunctionName,
    pub body: Vec<Expression>,
}

impl Compile for FunctionDeclaration {
    fn compile<'ctx>(self, compiler: &mut Compiler<'ctx>) -> Result<(), compiler::Error> {
        let function_type = compiler.context.void_type().fn_type(&[], false);
        let function = compiler
            .module
            .add_function(self.name.as_str(), function_type, None);

        let basic_block = compiler.context.append_basic_block(function, "entry");
        compiler.builder.position_at_end(basic_block);
        for expr in self.body {
            expr.compile(compiler)?;
        }
        compiler.builder.build_return(None);

        Ok(())
    }
}
