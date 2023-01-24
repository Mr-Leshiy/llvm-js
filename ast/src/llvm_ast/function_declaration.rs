use super::{Expression, Identifier};
use crate::{Compiler, CompilerError, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableFunctionDeclaration {
    pub name: Identifier,
    pub args_num: u32,
}

impl VariableFunctionDeclaration {
    pub fn compile(self, _compiler: &mut Compiler) -> Result<(), CompilerError> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: Vec<Expression>,
}

impl FunctionDeclaration {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let function = Function::new(
            compiler,
            &String::from(self.name.clone()),
            self.args,
            self.body,
        )?;
        compiler.insert_function(self.name, function)
    }
}
