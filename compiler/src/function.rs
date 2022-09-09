use crate::{Compile, Compiler, Error, Variable};
use inkwell::{
    attributes::{Attribute, AttributeLoc},
    values::FunctionValue,
    AddressSpace,
};
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
                Variable::get_type(compiler)
                    .ptr_type(AddressSpace::Generic)
                    .into()
            })
            .collect();
        let function_type = compiler
            .context
            .void_type()
            .fn_type(args_type.as_slice(), false);
        let function = compiler.module.add_function(name, function_type, None);

        // define argument attributes
        for i in 0..args_type.len() {
            let attribute = compiler.context.create_type_attribute(
                Attribute::get_named_enum_kind_id("byval"),
                Variable::get_type(compiler).into(),
            );
            function.add_attribute(AttributeLoc::Param(i as u32), attribute)
        }

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
        compiler
            .builder
            .build_return(Some(&compiler.context.i32_type().const_int(0, false)));
        Ok(())
    }

    pub fn generate_call(
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
            .build_call(self.function, vec.as_slice(), "call");
        Ok(())
    }
}
