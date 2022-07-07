use super::FunctionName;
use crate::compiler::{self, Compile, Compiler};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: FunctionName,
    // TODO: add args field
}

impl Compile for FunctionCall {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        // handle predifined functions
        if self.name == "printf" {
            Ok(())
        } else {
            match compiler.functions.get(&self.name).cloned() {
                Some(function) => {
                    compiler.builder.build_call(function, &[], "call");
                    Ok(())
                }
                None => Err(compiler::Error::UndefinedFunction(self.name)),
            }
        }
    }
}
