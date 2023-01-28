use super::{Expression, Identifier};
use crate::{Compiler, CompilerError, Function};
use compiler::Variable;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableFunctionDeclaration {
    pub name: Identifier,
    pub args_num: u32,
}

impl VariableFunctionDeclaration {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        let function = compiler.get_function(self.name.clone())?;
        let var = Variable::new_function(compiler, &function, false)?;
        compiler.insert_variable(self.name, var)?;
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
