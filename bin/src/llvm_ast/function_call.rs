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
            match compiler.functions.get(&self.name).cloned() {
                Some(function) => {
                    let args_num = function.get_type().get_param_types().len();
                    let mut vec = Vec::with_capacity(args_num);
                    for (i, arg_name) in self.args.into_iter().enumerate() {
                        if i >= args_num {
                            break;
                        }
                        let pointer = compiler
                            .variables
                            .get(&arg_name)
                            .cloned()
                            .ok_or(compiler::Error::UndefinedVariable(arg_name))?;
                        vec.push(pointer.into());
                    }

                    compiler
                        .builder
                        .build_call(function, vec.as_slice(), "call");
                    Ok(())
                }
                None => Err(compiler::Error::UndefinedFunction(self.name)),
            }
        }
    }
}
