use crate::{Compile, Compiler, Error, Variable};
use inkwell::{
    values::{ArrayValue, FunctionValue},
    AddressSpace,
};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct Function<'ctx, T> {
    pub(crate) arg_names: Vec<T>,
    pub(crate) function: FunctionValue<'ctx>,
    pub(crate) args: Option<ArrayValue<'ctx>>,
    pub(crate) variables: HashMap<T, Variable<'ctx>>,
}

impl<'ctx, T> Function<'ctx, T>
where
    T: Clone + Hash + PartialEq + Eq,
{
    pub fn new<Expr: Compile<T, Output = bool>>(
        compiler: &mut Compiler<'ctx, T>,
        name: &str,
        arg_names: Vec<T>,
        body: Vec<Expr>,
    ) -> Result<Self, Error<T>> {
        let var_type = compiler.inkwell_context.variable_type;
        let function_type =
            var_type.fn_type(&[var_type.ptr_type(AddressSpace::from(0)).into()], false);
        let function = compiler
            .inkwell_context
            .module
            .add_function(name, function_type, None);

        // generate body
        let basic_block = compiler
            .inkwell_context
            .context
            .append_basic_block(function, "entry");
        compiler
            .inkwell_context
            .builder
            .position_at_end(basic_block);

        // args
        let args = function
            .get_params()
            .first()
            .expect("")
            .into_pointer_value();
        let args = compiler
            .inkwell_context
            .builder
            .build_bitcast(
                args,
                var_type
                    .array_type(arg_names.len().try_into().unwrap())
                    .ptr_type(AddressSpace::from(0)),
                "",
            )
            .into_pointer_value();
        let args = Some(
            compiler
                .inkwell_context
                .builder
                .build_load(args, "")
                .into_array_value(),
        );

        let func = Self {
            function,
            args,
            arg_names,
            variables: HashMap::new(),
        };

        compiler.cur_function = Some(func.clone());

        let mut is_returned = false;
        for expr in body {
            let is_return = expr.compile(compiler)?;
            if is_return {
                is_returned = true;
                break;
            }
        }
        if !is_returned {
            let ret = Variable::new_undefined(compiler, true);
            compiler
                .inkwell_context
                .builder
                .build_return(Some(&ret.value));
        }

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

    pub(crate) fn get_variable(
        &self,
        compiler: &Compiler<'ctx, T>,
        name: T,
    ) -> Result<Variable<'ctx>, Error<T>> {
        if let Some(args) = self.args {
            // firstly look into the function arguments
            for (i, arg_name) in self.arg_names.iter().enumerate() {
                if name.eq(arg_name) {
                    let arg = compiler
                        .inkwell_context
                        .builder
                        .build_extract_value(args, i.try_into().unwrap(), "")
                        .unwrap()
                        .into_pointer_value();
                    return Ok(Variable {
                        value: arg,
                        is_tmp: false,
                    });
                }
            }
        }

        self.variables
            .get(&name)
            .cloned()
            .ok_or(Error::UndefinedVariable(name))
    }

    pub fn return_value(compiler: &mut Compiler<'ctx, T>, ret: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_return(Some(&ret.value));
    }
}
