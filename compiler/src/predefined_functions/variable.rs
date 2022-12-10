use super::{Compiler, PredefineFunctionName};
use crate::Variable;
use inkwell::{
    module::Linkage,
    values::{FunctionValue, IntValue},
    AddressSpace,
};

#[derive(Clone)]
pub struct AllocateFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AllocateFn<'ctx> {
    const NAME: &'static str = "allocate";
}

impl<'ctx> AllocateFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = var_type.fn_type(&[], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));

        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>) -> Variable<'ctx> {
        let value = compiler
            .builder
            .build_call(self.func, &[], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable { value }
    }
}

#[derive(Clone)]
pub struct DeallocateFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for DeallocateFn<'ctx> {
    const NAME: &'static str = "deallocate";
}

impl<'ctx> DeallocateFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));

        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetUndefinedFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetUndefinedFn<'ctx> {
    const NAME: &'static str = "set_undefined";
}

impl<'ctx> SetUndefinedFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNullFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetNullFn<'ctx> {
    const NAME: &'static str = "set_null";
}

impl<'ctx> SetNullFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNaNFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetNaNFn<'ctx> {
    const NAME: &'static str = "set_nan";
}

impl<'ctx> SetNaNFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetObjectFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetObjectFn<'ctx> {
    const NAME: &'static str = "set_object";
}

impl<'ctx> SetObjectFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetArrayFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetArrayFn<'ctx> {
    const NAME: &'static str = "set_array";
}

impl<'ctx> SetArrayFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetInfinityFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetInfinityFn<'ctx> {
    const NAME: &'static str = "set_infinity";
}

impl<'ctx> SetInfinityFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNegInfinityFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetNegInfinityFn<'ctx> {
    const NAME: &'static str = "set_neginfinity";
}

impl<'ctx> SetNegInfinityFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNumberFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetNumberFn<'ctx> {
    const NAME: &'static str = "set_number";
}

impl<'ctx> SetNumberFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let number_type = compiler.context.f64_type();

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into(), number_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, literal: f64) {
        compiler.builder.build_call(
            self.func,
            &[
                val.value.into(),
                compiler.context.f64_type().const_float(literal).into(),
            ],
            "",
        );
    }
}

#[derive(Clone)]
pub struct SetBooleanFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetBooleanFn<'ctx> {
    const NAME: &'static str = "set_boolean";
}

impl<'ctx> SetBooleanFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let boolean_type = compiler.context.bool_type();

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into(), boolean_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, literal: bool) {
        compiler.builder.build_call(
            self.func,
            &[
                val.value.into(),
                compiler
                    .context
                    .bool_type()
                    .const_int(literal.into(), false)
                    .into(),
            ],
            "",
        );
    }
}

#[derive(Clone)]
pub struct SetStringFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetStringFn<'ctx> {
    const NAME: &'static str = "set_string";
}

impl<'ctx> SetStringFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into(), string_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, literal: &str) {
        let literal = compiler
            .builder
            .build_global_string_ptr(literal, "")
            .as_pointer_value();
        compiler
            .builder
            .build_call(self.func, &[val.value.into(), literal.into()], "");
    }
}

#[derive(Clone)]
pub struct SetVariableFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for SetVariableFn<'ctx> {
    const NAME: &'static str = "set_variable";
}

impl<'ctx> SetVariableFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into(), var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val1: &Variable<'ctx>,
        val2: &Variable<'ctx>,
    ) {
        compiler
            .builder
            .build_call(self.func, &[val1.value.into(), val2.value.into()], "");
    }
}

#[derive(Clone)]
pub struct GetBooleanFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for GetBooleanFn<'ctx> {
    const NAME: &'static str = "get_boolean";
}

impl<'ctx> GetBooleanFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .i8_type()
            .fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
    ) -> IntValue<'ctx> {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value()
    }
}

#[derive(Clone)]
pub struct AddPropertyByStrFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AddPropertyByStrFn<'ctx> {
    const NAME: &'static str = "add_property_by_str";
}

impl<'ctx> AddPropertyByStrFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);

        let function_type = compiler.context.void_type().fn_type(
            &[var_type.into(), string_type.into(), var_type.into()],
            false,
        );
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &str,
        value: &Variable<'ctx>,
    ) {
        let key = compiler
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        compiler.builder.build_call(
            self.func,
            &[val.value.into(), key.into(), value.value.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct AddPropertyByVarFn<'ctx> {
    // TODO rename
    _func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AddPropertyByVarFn<'ctx> {
    const NAME: &'static str = "add_property_by_var";
}

impl<'ctx> AddPropertyByVarFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into(), var_type.into(), var_type.into()], false);
        let _func =
            compiler
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { _func }
    }

    // TODO rename
    pub(crate) fn _call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &Variable<'ctx>,
        value: &Variable<'ctx>,
    ) {
        compiler.builder.build_call(
            self._func,
            &[val.value.into(), key.value.into(), value.value.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct GetPropertyByStrFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for GetPropertyByStrFn<'ctx> {
    const NAME: &'static str = "get_property_by_str";
}

impl<'ctx> GetPropertyByStrFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);
        let boolean_type = compiler.context.bool_type();

        let function_type = var_type.fn_type(
            &[var_type.into(), string_type.into(), boolean_type.into()],
            false,
        );
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &str,
        allocate: bool,
    ) -> Variable<'ctx> {
        let key = compiler
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        let value = compiler
            .builder
            .build_call(
                self.func,
                &[
                    val.value.into(),
                    key.into(),
                    compiler
                        .context
                        .bool_type()
                        .const_int(allocate.into(), false)
                        .into(),
                ],
                "",
            )
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable { value }
    }
}

#[derive(Clone)]
pub struct GetPropertyByVarFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for GetPropertyByVarFn<'ctx> {
    const NAME: &'static str = "get_property_by_var";
}

impl<'ctx> GetPropertyByVarFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let boolean_type = compiler.context.bool_type();

        let function_type = var_type.fn_type(
            &[var_type.into(), var_type.into(), boolean_type.into()],
            false,
        );
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &Variable<'ctx>,
        allocate: bool,
    ) -> Variable<'ctx> {
        let value = compiler
            .builder
            .build_call(
                self.func,
                &[
                    val.value.into(),
                    key.value.into(),
                    compiler
                        .context
                        .bool_type()
                        .const_int(allocate.into(), false)
                        .into(),
                ],
                "",
            )
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable { value }
    }
}

#[derive(Clone)]
pub struct RemovePropertyFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for RemovePropertyFn<'ctx> {
    const NAME: &'static str = "remove_property";
}

impl<'ctx> RemovePropertyFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into(), string_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, key: &str) {
        let key = compiler
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        compiler
            .builder
            .build_call(self.func, &[val.value.into(), key.into()], "");
    }
}
