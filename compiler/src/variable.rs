use super::Compiler;
use crate::Function;
use inkwell::values::PointerValue;

#[derive(Clone)]
pub struct Variable<'ctx> {
    pub(crate) value: PointerValue<'ctx>,
    pub(crate) is_tmp: bool,
}

impl<'ctx> Variable<'ctx> {
    pub const TYPE_NAME: &'static str = "Variable";

    pub(crate) fn new<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let allocate_fn = compiler.predefined_functions().allocate();
        allocate_fn.call(compiler)
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn is_tmp(&self) -> bool {
        self.is_tmp
    }

    pub fn deallocate<T>(&self, compiler: &Compiler<'ctx, T>) {
        let deallocate_fn = compiler.predefined_functions().deallocate();
        deallocate_fn.call(compiler, self);
    }

    pub fn declare_global<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.inkwell_context.variable_type;
        let value = compiler
            .inkwell_context
            .module
            .add_global(var_type, None, "")
            .as_pointer_value();
        Self {
            value,
            is_tmp: false,
        }
    }

    pub fn new_undefined<T>(compiler: &Compiler<'ctx, T>, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_undefined_fn = compiler.predefined_functions().set_undefined();
        set_undefined_fn.call(compiler, &variable);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_null<T>(compiler: &Compiler<'ctx, T>, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_null_fn = compiler.predefined_functions().set_null();
        set_null_fn.call(compiler, &variable);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_object<T>(compiler: &Compiler<'ctx, T>, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_empty_object_fn = compiler.predefined_functions().set_empty_object();
        set_empty_object_fn.call(compiler, &variable);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_array<T>(compiler: &Compiler<'ctx, T>, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_empty_array_fn = compiler.predefined_functions().set_empty_array();
        set_empty_array_fn.call(compiler, &variable);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_nan<T>(compiler: &Compiler<'ctx, T>, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_nan_fn = compiler.predefined_functions().set_nan();
        set_nan_fn.call(compiler, &variable);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_infinity<T>(compiler: &Compiler<'ctx, T>, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_infinity_fn = compiler.predefined_functions().set_infinity();
        set_infinity_fn.call(compiler, &variable);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_neginfinity<T>(compiler: &Compiler<'ctx, T>, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_neginfinity_fn = compiler.predefined_functions().set_neginfinity();
        set_neginfinity_fn.call(compiler, &variable);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_number<T>(compiler: &Compiler<'ctx, T>, number: f64, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_number_fn = compiler.predefined_functions().set_number();
        set_number_fn.call(compiler, &variable, number);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_boolean<T>(compiler: &Compiler<'ctx, T>, boolean: bool, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_boolean_fn = compiler.predefined_functions().set_boolean();
        set_boolean_fn.call(compiler, &variable, boolean);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_string<T>(compiler: &Compiler<'ctx, T>, string: &str, is_tmp: bool) -> Self {
        let mut variable = Self::new(compiler);
        let set_string_fn = compiler.predefined_functions().set_string();
        set_string_fn.call(compiler, &variable, string);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_function<T>(
        compiler: &Compiler<'ctx, T>,
        function: &Function<'ctx, T>,
        is_tmp: bool,
    ) -> Self {
        let mut variable = Self::new(compiler);
        let set_function_fn = compiler.predefined_functions().set_function();
        set_function_fn.call(compiler, &variable, function);
        variable.is_tmp = is_tmp;
        variable
    }

    pub fn new_variable<T>(compiler: &Compiler<'ctx, T>, variable2: &Self) -> Self {
        let variable1 = Self::new(compiler);
        variable1.assign_variable(compiler, variable2);
        variable1
    }

    pub fn assign_variable<T>(&self, compiler: &Compiler<'ctx, T>, variable: &Self) {
        let set_variable_fn = compiler.predefined_functions().set_variable();
        set_variable_fn.call(compiler, self, variable);
    }
}

impl<'ctx> Variable<'ctx> {
    #[must_use]
    pub fn function_call<T>(&self, compiler: &Compiler<'ctx, T>, args: &[Self]) -> Self {
        let function_call_fn = compiler.predefined_functions().function_call();
        let ret = function_call_fn.call(compiler, self, args);
        ret
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn add_property_by_boolean<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: bool,
        prop: &Self,
        move_prop: bool,
    ) {
        let add_property_fn = compiler.predefined_functions().add_property_by_boolean();
        add_property_fn.call(compiler, self, key, prop, move_prop);
    }

    pub fn add_property_by_number<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: f64,
        prop: &Self,
        move_prop: bool,
    ) {
        let add_property_fn = compiler.predefined_functions().add_property_by_number();
        add_property_fn.call(compiler, self, key, prop, move_prop);
    }

    pub fn add_property_by_str<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: &str,
        prop: &Self,
        move_prop: bool,
    ) {
        let add_property_fn = compiler.predefined_functions().add_property_by_str();
        add_property_fn.call(compiler, self, key, prop, move_prop);
    }

    pub fn add_property_by_var<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: &Variable<'ctx>,
        prop: &Self,
        move_prop: bool,
    ) {
        let add_property_fn = compiler.predefined_functions().add_property_by_var();
        add_property_fn.call(compiler, self, key, prop, move_prop);
    }

    #[must_use]
    pub fn get_property_by_boolean<T>(&self, compiler: &Compiler<'ctx, T>, key: bool) -> Self {
        let get_property_fn = compiler.predefined_functions().get_property_by_boolean();
        get_property_fn.call(compiler, self, key)
    }

    #[must_use]
    pub fn get_property_by_number<T>(&self, compiler: &Compiler<'ctx, T>, key: f64) -> Self {
        let get_property_fn = compiler.predefined_functions().get_property_by_number();
        get_property_fn.call(compiler, self, key)
    }

    #[must_use]
    pub fn get_property_by_str<T>(&self, compiler: &Compiler<'ctx, T>, key: &str) -> Self {
        let get_property_fn = compiler.predefined_functions().get_property_by_str();
        get_property_fn.call(compiler, self, key)
    }

    #[must_use]
    pub fn get_property_by_var<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        key: &Variable<'ctx>,
    ) -> Self {
        let get_property_fn = compiler.predefined_functions().get_property_by_var();
        get_property_fn.call(compiler, self, key)
    }

    pub fn remove_property<T>(&self, compiler: &Compiler<'ctx, T>, key: &str) {
        let remove_property_fn = compiler.predefined_functions().remove_property();
        remove_property_fn.call(compiler, self, key);
    }
}
