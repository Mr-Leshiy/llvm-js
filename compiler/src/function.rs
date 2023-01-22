use crate::{Compile, Compiler, Error, Variable};
use inkwell::{values::FunctionValue, AddressSpace};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct Function<'ctx, T> {
    pub(crate) arg_names: Vec<T>,
    pub(crate) function: FunctionValue<'ctx>,
    pub(crate) variables: HashMap<T, Variable<'ctx>>,
}

impl<'ctx, T> Function<'ctx, T>
where
    T: Clone + Hash + PartialEq + Eq,
{
    fn generate_body<Expr: Compile<T, Output = bool>>(
        compiler: &mut Compiler<'ctx, T>,
        body: Vec<Expr>,
    ) -> Result<(), Error<T>> {
        let basic_block = compiler
            .context
            .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "entry");
        compiler.builder.position_at_end(basic_block);
        let mut is_returned = false;
        for expr in body {
            let is_return = expr.compile(compiler)?;
            if is_return {
                is_returned = true;
                break;
            }
        }
        if !is_returned {
            let ret = Variable::new_undefined(compiler, true)?;
            compiler.builder.build_return(Some(&ret.value));
        }
        Ok(())
    }

    pub fn new<Expr: Compile<T, Output = bool>>(
        compiler: &mut Compiler<'ctx, T>,
        name: &str,
        arg_names: Vec<T>,
        body: Vec<Expr>,
    ) -> Result<Self, Error<T>> {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::from(0));
        let args_type: Vec<_> = arg_names.iter().map(|_| var_type.into()).collect();
        let function_type = var_type.fn_type(args_type.as_slice(), false);
        let function = compiler.module.add_function(name, function_type, None);

        let func = Self {
            function,
            arg_names,
            variables: HashMap::new(),
        };

        compiler.cur_function = Some(func.clone());
        Self::generate_body(compiler, body)?;
        Ok(func)
    }

    pub(crate) fn insert_variable(
        &mut self,
        name: T,
        variable: Variable<'ctx>,
    ) -> Result<(), Error<T>> {
        match self.variables.insert(name.clone(), variable) {
            None => Ok(()),
            Some(_) => Err(Error::AlreadyDeclaredVariable(name)),
        }
    }

    pub(crate) fn get_variable(&self, name: T) -> Result<Variable<'ctx>, Error<T>> {
        // firstly look into the function arguments
        for (i, arg_name) in self.arg_names.iter().enumerate() {
            if name.eq(arg_name) {
                let arg = self
                    .function
                    .get_params()
                    .get(i)
                    .expect("")
                    .into_pointer_value();
                return Ok(Variable {
                    value: arg,
                    is_tmp: false,
                });
            }
        }

        self.variables
            .get(&name)
            .cloned()
            .ok_or(Error::UndefinedVariable(name))
    }

    pub fn return_value(compiler: &mut Compiler<'ctx, T>, ret: &Variable<'ctx>) {
        compiler.builder.build_return(Some(&ret.value));
    }

    pub fn call(
        &self,
        compiler: &mut Compiler<'ctx, T>,
        args: Vec<Variable<'ctx>>,
    ) -> Result<Variable<'ctx>, Error<T>> {
        let args_num = self.function.get_type().get_param_types().len();
        let mut vec = Vec::with_capacity(args_num);
        for (i, arg) in args.into_iter().enumerate() {
            if i >= args_num {
                break;
            }

            vec.push(arg.value.into());
        }

        let value = compiler
            .builder
            .build_call(self.function, vec.as_slice(), "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Ok(Variable {
            value,
            is_tmp: true,
        })
    }
}
