use super::Compiler;
use crate::{
    Error,
};
use inkwell::{
    types::StructType,
    values::{PointerValue},
};

#[derive(Clone)]
pub struct Variable<'ctx> {
    pub(crate) value: PointerValue<'ctx>,
}


impl<'ctx> Variable<'ctx> {
    pub(crate) fn new<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let allocate_fn = compiler.predefined_functions().get_allocate()?;
        let value = allocate_fn.call(compiler);
        Ok(Self { value })
    }

    pub(crate) fn get_type<T>(compiler: &Compiler<'ctx, T>) -> StructType<'ctx> {
        compiler.context.opaque_struct_type("VariableType")
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn new_number<T>(compiler: &Compiler<'ctx, T>, number: f64) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        variable.assign_number(compiler, number)?;
        Ok(variable)
    }

    pub fn assign_number<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        literal: f64,
    ) -> Result<(), Error<T>> {
        let set_number_fn = compiler.predefined_functions().get_set_number()?;
        set_number_fn.call(compiler, self, literal);
        Ok(())
    }

    pub fn new_boolean<T>(compiler: &Compiler<'ctx, T>, boolean: bool) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        variable.assign_boolean(compiler, boolean)?;
        Ok(variable)
    }

    pub fn assign_boolean<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        literal: bool,
    ) -> Result<(), Error<T>> {
        let set_boolean_fn = compiler.predefined_functions().get_set_boolean()?;
        set_boolean_fn.call(compiler, self, literal);
        Ok(())
    }

    pub fn new_string<T>(compiler: &Compiler<'ctx, T>, string: &str) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        variable.assign_string(compiler, string)?;
        Ok(variable)
    }

    pub fn assign_string<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        literal: &str,
    ) -> Result<(), Error<T>> {
        let set_string_fn = compiler.predefined_functions().get_set_string()?;
        set_string_fn.call(compiler, self, literal);
        Ok(())
    }

    pub fn new_variable<T>(
        compiler: &Compiler<'ctx, T>,
        variable2: &Self,
    ) -> Result<Self, Error<T>> {
        let variable1 = Self::new(compiler)?;
        variable1.assign_variable(compiler, variable2)?;
        Ok(variable1)
    }

    pub fn assign_variable<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        variable: &Self,
    ) -> Result<(), Error<T>> {
        let set_variable_fn = compiler.predefined_functions().get_set_variable()?;
        set_variable_fn.call(compiler, self, variable);
        Ok(())
    }
}
