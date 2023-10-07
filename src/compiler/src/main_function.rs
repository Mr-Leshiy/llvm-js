use crate::{Compile, Compiler, Error, Function};
use inkwell::values::FunctionValue;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct MainFunction;

impl<'ctx> MainFunction {
    fn generate_body<Expr: Compile<T>, T: Clone + Hash + PartialEq + Eq>(
        compiler: &mut Compiler<'ctx, T>,
        function: FunctionValue<'ctx>,
        body: Vec<Expr>,
    ) -> Result<(), Error<T>> {
        compiler.cur_function = Some(Function {
            function,
            args: None,
            arg_names: Vec::new(),
            variables: HashMap::new(),
        });

        let basic_block = compiler
            .inkwell_context
            .context
            .append_basic_block(function, "entry");
        compiler
            .inkwell_context
            .builder
            .position_at_end(basic_block);
        for expr in body {
            expr.compile(compiler)?;
        }
        compiler.inkwell_context.builder.build_return(Some(
            &compiler
                .inkwell_context
                .context
                .i32_type()
                .const_int(0, false),
        ));
        Ok(())
    }

    pub fn define<Expr: Compile<T>, T: Clone + Hash + PartialEq + Eq>(
        compiler: &mut Compiler<'ctx, T>,
        body: Vec<Expr>,
    ) -> Result<(), Error<T>> {
        let function_type = compiler
            .inkwell_context
            .context
            .i32_type()
            .fn_type(&[], false);
        let function = compiler
            .inkwell_context
            .module
            .add_function("main", function_type, None);
        Self::generate_body(compiler, function, body)?;
        Ok(())
    }
}
