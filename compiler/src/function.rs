use crate::{Compile, Compiler, Error, Variable};
use inkwell::{values::FunctionValue, AddressSpace};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct Function<'ctx, T> {
    pub(super) arg_names: Vec<T>,
    pub(super) function: FunctionValue<'ctx>,
    pub(super) variables: HashMap<T, Variable<'ctx>>,
}

impl<'ctx, T> Function<'ctx, T>
where
    T: Clone + Hash + PartialEq + Eq,
{
    pub fn new(compiler: &mut Compiler<'ctx, T>, name: &str, arg_names: Vec<T>) -> Self {
        let args_type: Vec<_> = arg_names
            .iter()
            .map(|_| {
                compiler
                    .variable_type
                    .ptr_type(AddressSpace::Generic)
                    .into()
            })
            .collect();
        let function_type = compiler
            .context
            .void_type()
            .fn_type(args_type.as_slice(), false);
        let function = compiler.module.add_function(name, function_type, None);

        Self {
            function,
            arg_names,
            variables: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: T) -> Result<Variable<'ctx>, Error<T>> {
        // firstly look into the function arguments
        for (i, arg_name) in self.arg_names.iter().enumerate() {
            if name.eq(arg_name) {
                let arg = self
                    .function
                    .get_params()
                    .get(i)
                    .expect("")
                    .into_pointer_value();
                return Ok(Variable { value: arg });
            }
        }

        self.variables
            .get(&name)
            .cloned()
            .ok_or(Error::UndefinedVariable(name))
    }

    pub fn insert_variable(&mut self, name: T, variable: Variable<'ctx>) -> Result<(), Error<T>> {
        match self.variables.insert(name.clone(), variable) {
            None => Ok(()),
            Some(_) => Err(Error::AlreadyDeclaredVariable(name)),
        }
    }

    // TODO: move this code inside new function
    pub fn generate_body<Expr: Compile<T>>(
        &mut self,
        compiler: &mut Compiler<'ctx, T>,
        body: Vec<Expr>,
    ) -> Result<(), Error<T>> {
        let basic_block = compiler.context.append_basic_block(self.function, "entry");
        compiler.builder.position_at_end(basic_block);
        for expr in body {
            expr.compile(compiler, self)?;
        }
        compiler.builder.build_return(None);
        Ok(())
    }

    pub fn call(
        &self,
        compiler: &mut Compiler<'ctx, T>,
        args: Vec<Variable<'ctx>>,
    ) -> Result<(), Error<T>> {
        let args_num = self.function.get_type().get_param_types().len();
        let mut vec = Vec::with_capacity(args_num);
        for (i, arg) in args.into_iter().enumerate() {
            if i >= args_num {
                break;
            }

            vec.push(arg.value.into());
        }

        compiler
            .builder
            .build_call(self.function, vec.as_slice(), "");
        Ok(())
    }
}
