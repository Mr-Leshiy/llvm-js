use super::{FunctionName, VariableName};
use compiler::{self, Compile, Compiler};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: FunctionName,
    pub args: Vec<VariableName>,
}

impl Compile for FunctionCall {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        // handle predifined functions
        if self.name == "printf" {
            // TODO refactor
            let printf = compiler.get_printf()?;
            let arg_name = self
                .args
                .into_iter()
                .next()
                .ok_or(compiler::Error::NotEnoughArguments)?;
            let val = compiler
                .variables
                .get(&arg_name)
                .cloned()
                .ok_or(compiler::Error::UndefinedVariable(arg_name))?;

            printf.print(compiler, val)
        } else {
            let function = compiler.get_function(self.name)?;
            function.generate_call(compiler, self.args)
        }
    }
}
