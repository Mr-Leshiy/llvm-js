use super::{FunctionName, VariableName};
use compiler::{self, Compile, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: FunctionName,
    pub args: Vec<VariableName>,
}

impl Compile for FunctionCall {
    fn compile(self, compiler: &mut Compiler, _: &Function) -> Result<(), compiler::Error> {
        let function = compiler.get_function(self.name)?;
        function.generate_call(compiler, self.args)
    }
}
