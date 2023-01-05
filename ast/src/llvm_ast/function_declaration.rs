use super::{Expression, Identifier};
use crate::CompilerError;
use compiler::{Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: Vec<Expression>,
}

impl FunctionDeclaration {
    pub fn compile(self, compiler: &mut Compiler<Identifier>) -> Result<(), CompilerError> {
        let mut function = Function::new(compiler, &String::from(self.name.clone()), self.args);
        function.generate_body(compiler, self.body)?;

        compiler.insert_function(self.name, function)
    }
}
