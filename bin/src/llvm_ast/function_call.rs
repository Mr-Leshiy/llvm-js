use super::{FunctionName, VariableName};
use compiler::{self, Compile, Compiler};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: FunctionName,
    pub args: Vec<VariableName>,
}

impl Compile for FunctionCall {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let function = compiler.get_function(self.name)?;
        function.generate_call(compiler, self.args)
    }
}
