use super::{expression::Expression, FunctionDeclaration};
use crate::compiler::{self, Compile, Compiler};

pub struct Program {
    pub functions: Vec<FunctionDeclaration>,
    pub body: Vec<Expression>,
}

impl Compile for Program {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        for func in self.functions {
            func.compile(compiler)?;
        }

        // define main function
        let function_type = compiler.context.void_type().fn_type(&[], false);
        let function = compiler.module.add_function("main", function_type, None);
        let basic_block = compiler.context.append_basic_block(function, "entry");
        compiler.builder.position_at_end(basic_block);

        for expr in self.body {
            expr.compile(compiler)?;
        }

        compiler.builder.build_return(None);

        Ok(())
    }
}
