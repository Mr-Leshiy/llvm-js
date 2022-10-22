use super::Compiler;
use crate::Error;
use inkwell::values::PointerValue;

#[derive(Clone)]
pub struct Variable<'ctx> {
    pub(crate) value: PointerValue<'ctx>,
}

impl<'ctx> Variable<'ctx> {
    pub const TYPE_NAME: &'static str = "Variable";

    pub(crate) fn new<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let allocate_fn = compiler.predefined_functions()?.allocate();
        Ok(allocate_fn.call(compiler))
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn new_undefined<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_undefined_fn = compiler.predefined_functions()?.set_undefined();
        set_undefined_fn.call(compiler, &variable);
        Ok(variable)
    }

    pub fn new_null<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_null_fn = compiler.predefined_functions()?.set_null();
        set_null_fn.call(compiler, &variable);
        Ok(variable)
    }

    pub fn set_object<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_object_fn = compiler.predefined_functions()?.set_object();
        set_object_fn.call(compiler, &variable);
        Ok(variable)
    }

    pub fn new_nan<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_nan_fn = compiler.predefined_functions()?.set_nan();
        set_nan_fn.call(compiler, &variable);
        Ok(variable)
    }

    pub fn new_infinity<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_infinity_fn = compiler.predefined_functions()?.set_infinity();
        set_infinity_fn.call(compiler, &variable);
        Ok(variable)
    }

    pub fn new_neginfinity<T>(compiler: &Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_neginfinity_fn = compiler.predefined_functions()?.set_neginfinity();
        set_neginfinity_fn.call(compiler, &variable);
        Ok(variable)
    }

    pub fn new_number<T>(compiler: &Compiler<'ctx, T>, number: f64) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_number_fn = compiler.predefined_functions()?.set_number();
        set_number_fn.call(compiler, &variable, number);
        Ok(variable)
    }

    pub fn new_boolean<T>(compiler: &Compiler<'ctx, T>, boolean: bool) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_boolean_fn = compiler.predefined_functions()?.set_boolean();
        set_boolean_fn.call(compiler, &variable, boolean);
        Ok(variable)
    }

    pub fn new_string<T>(compiler: &Compiler<'ctx, T>, string: &str) -> Result<Self, Error<T>> {
        let variable = Self::new(compiler)?;
        let set_string_fn = compiler.predefined_functions()?.set_string();
        set_string_fn.call(compiler, &variable, string);
        Ok(variable)
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
        let set_variable_fn = compiler.predefined_functions()?.set_variable();
        set_variable_fn.call(compiler, self, variable);
        Ok(())
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn add_property<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: &str,
        value: &Self,
    ) -> Result<(), Error<T>> {
        let add_property_fn = compiler.predefined_functions()?.add_property();
        add_property_fn.call(compiler, self, key, value);
        Ok(())
    }

    pub fn get_property_by_str<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: &str,
    ) -> Result<Self, Error<T>> {
        let get_property_fn = compiler.predefined_functions()?.get_property_by_str();
        Ok(get_property_fn.call(compiler, self, key))
    }

    pub fn get_property_by_var<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: &Variable<'ctx>,
    ) -> Result<Self, Error<T>> {
        let get_property_fn = compiler.predefined_functions()?.get_property_by_var();
        Ok(get_property_fn.call(compiler, self, key))
    }

    pub fn remove_property<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: &str,
    ) -> Result<(), Error<T>> {
        let remove_property_fn = compiler.predefined_functions()?.remove_property();
        remove_property_fn.call(compiler, self, key);
        Ok(())
    }
}
