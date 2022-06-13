use super::{Compile, CompileResult, Compiler, Error};
use crate::ast::Program;
use inkwell::module::Module;

impl CompileResult for () {
    fn to_string(&self) -> String {
        "".to_string()
    }
}

impl<'ctx> Compile<'ctx> for Program {
    type Output = ();

    fn compile(
        &self,
        compiler: &mut Compiler<'ctx>,
        module: &Module<'ctx>,
    ) -> Result<Self::Output, Error> {
        // create entry point main function
        let func = module.add_function(
            "main",
            compiler.context.void_type().fn_type(&[], false),
            None,
        );
        let block = compiler.context.append_basic_block(func, "entry");
        compiler.builder.position_at_end(block);

        for expr in &self.body {
            expr.compile(compiler, module)?;
        }
        compiler.builder.build_return(None);

        Ok(())
    }
}
