use crate::{Compile, Compiler, Error, Variable};
use inkwell::values::FunctionValue;

#[derive(Clone)]
pub struct Function<'ctx> {
    pub(super) function: FunctionValue<'ctx>,
}

impl<'ctx> Function<'ctx> {
    pub fn new(compiler: &mut Compiler<'ctx>, name: &str, args: &Vec<String>) -> Self {
        let args: Vec<_> = args
            .iter()
            .map(|_| Variable::get_type(compiler).into())
            .collect();
        let function_type = compiler.context.void_type().fn_type(args.as_slice(), false);
        let function = compiler.module.add_function(name, function_type, None);

        Self { function }
    }

    pub fn generate_body<T: Compile>(
        &self,
        compiler: &mut Compiler<'ctx>,
        body: Vec<T>,
    ) -> Result<(), Error> {
        let basic_block = compiler.context.append_basic_block(self.function, "entry");
        compiler.builder.position_at_end(basic_block);
        for expr in body {
            expr.compile(compiler, self)?;
        }
        compiler.builder.build_return(None);
        Ok(())
    }

    pub fn generate_call(
        &self,
        compiler: &mut Compiler<'ctx>,
        args_names: Vec<String>,
    ) -> Result<(), Error> {
        let args_num = self.function.get_type().get_param_types().len();
        let mut vec = Vec::with_capacity(args_num);
        for (i, arg_name) in args_names.into_iter().enumerate() {
            if i >= args_num {
                break;
            }
            let variable = compiler
                .variables
                .get(&arg_name)
                .cloned()
                .ok_or(Error::UndefinedVariable(arg_name))?;
            vec.push(variable.get_value(compiler).into());
        }

        compiler
            .builder
            .build_call(self.function, vec.as_slice(), "call");
        Ok(())
    }
}
