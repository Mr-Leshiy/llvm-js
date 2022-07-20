use super::{Expression, VariableName};
use compiler::{self, Compiler, Function};

pub type FunctionName = String;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: FunctionName,
    pub args: Vec<VariableName>,
    pub body: Vec<Expression>,
}

impl FunctionDeclaration {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let mut function = Function::new(compiler, &self.name, self.args);
        function.generate_body(compiler, self.body)?;

        compiler.insert_function(self.name, function)
    }
}
