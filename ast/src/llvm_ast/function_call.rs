use super::{FunctionName, VariableValue};
use compiler::{
    self,
    extern_functions::{printf::PrintFn, ExternFunctionName},
    Compile, Compiler, Function,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: FunctionName,
    pub args: Vec<VariableValue>,
}

impl Compile for FunctionCall {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
    ) -> Result<(), compiler::Error> {
        match self.name.as_str() {
            PrintFn::NAME => {
                let pritnf = compiler.extern_functions().get_print()?;
                // pritnf.print(compiler, cur_function, self.args)
                Ok(())
            }
            _ => {
                let function = compiler.get_function(self.name)?;
                // function.generate_call(compiler, cur_function, self.args)
                Ok(())
            }
        }
    }
}
