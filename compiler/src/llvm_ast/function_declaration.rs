use super::Expression;
use crate::compiler::{self, Compile, Compiler};

pub type FunctionName = String;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: FunctionName,
    // TODO: add args field
    pub body: Vec<Expression>,
}

impl Compile for FunctionDeclaration {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let function_type = compiler.context.void_type().fn_type(&[], false);
        let function = compiler
            .module
            .add_function(self.name.as_str(), function_type, None);

        match compiler.functions.insert(self.name.clone(), function) {
            None => Ok(()),
            Some(_) => Err(compiler::Error::AlreadyDeclaredFunction(self.name.clone())),
        }?;

        let basic_block = compiler.context.append_basic_block(function, "entry");
        compiler.builder.position_at_end(basic_block);
        for expr in self.body {
            expr.compile(compiler)?;
        }
        compiler.builder.build_return(None);

        Ok(())
    }
}
