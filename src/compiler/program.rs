use super::{Compile, Compiler, Error};
use crate::ast::Program;
use inkwell::module::Module;

impl<'ctx> Compile<'ctx> for Program {
    fn compile(self, compiler: &mut Compiler<'ctx>, module: &Module<'ctx>) -> Result<(), Error> {
        // create entry point main function
        let func = module.add_function(
            "main",
            compiler.context.void_type().fn_type(&[], false),
            None,
        );
        let block = compiler.context.append_basic_block(func, "entry");
        compiler.builder.position_at_end(block);

        for expr in self.body {
            expr.compile(compiler, module)?;
        }
        compiler.builder.build_return(None);

        Ok(())
    }
}
