use super::Expression;
use compiler::{self, Compile, Compiler, Function};

pub type FunctionName = String;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: FunctionName,
    // TODO: add args field
    pub body: Vec<Expression>,
}

impl Compile for FunctionDeclaration {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let function = Function::new(compiler, &self.name);
        function.generate_body(compiler, self.body)?;

        compiler.insert_function(self.name, function)
    }
}
