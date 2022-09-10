use crate::{Compile, Compiler, Error, Function};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct MainFunction<'ctx, T> {
    func: Function<'ctx, T>,
}

impl<'ctx, T> MainFunction<'ctx, T>
where
    T: Clone + Hash + PartialEq + Eq,
{
    pub fn new(compiler: &mut Compiler<'ctx, T>) -> Self {
        let function_type = compiler.context.i32_type().fn_type(&[], false);
        let function = compiler.module.add_function("main", function_type, None);

        Self {
            func: Function {
                function,
                arg_names: Vec::new(),
                variables: HashMap::new(),
            },
        }
    }

    // TODO: move this code inside new function
    pub fn generate_body<Expr: Compile<T>>(
        &mut self,
        compiler: &mut Compiler<'ctx, T>,
        body: Vec<Expr>,
    ) -> Result<(), Error<T>> {
        self.func.generate_body(compiler, body)
    }
}
