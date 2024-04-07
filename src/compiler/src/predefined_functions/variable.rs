use super::Compiler;
use crate::{Function, InkwellContext, Variable};
use inkwell::{
    module::Linkage,
    values::{FunctionValue, IntValue},
    AddressSpace,
};

#[derive(Clone)]
pub struct SetUndefinedFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetUndefinedFn<'ctx> {
    const NAME: &'static str = "set_undefined";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNullFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetNullFn<'ctx> {
    const NAME: &'static str = "set_null";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNaNFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetNaNFn<'ctx> {
    const NAME: &'static str = "set_nan";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetEmptyObjectFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetEmptyObjectFn<'ctx> {
    const NAME: &'static str = "set_empty_object";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetEmptyArrayFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetEmptyArrayFn<'ctx> {
    const NAME: &'static str = "set_empty_array";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetInfinityFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetInfinityFn<'ctx> {
    const NAME: &'static str = "set_infinity";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNegInfinityFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetNegInfinityFn<'ctx> {
    const NAME: &'static str = "set_neginfinity";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct SetNumberFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetNumberFn<'ctx> {
    const NAME: &'static str = "set_number";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let number_type = inkwell_context.context.f64_type();
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), number_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, literal: f64) {
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[
                val.value.into(),
                compiler
                    .inkwell_context
                    .context
                    .f64_type()
                    .const_float(literal)
                    .into(),
            ],
            "",
        );
    }
}

#[derive(Clone)]
pub struct SetBooleanFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetBooleanFn<'ctx> {
    const NAME: &'static str = "set_boolean";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let boolean_type = inkwell_context.context.bool_type();
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), boolean_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, literal: bool) {
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[
                val.value.into(),
                compiler
                    .inkwell_context
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

impl<'ctx> SetStringFn<'ctx> {
    const NAME: &'static str = "set_string";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let string_type = inkwell_context
            .context
            .i8_type()
            .ptr_type(AddressSpace::from(0));
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), string_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, literal: &str) {
        let literal = compiler
            .inkwell_context
            .builder
            .build_global_string_ptr(literal, "")
            .as_pointer_value();
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[val.value.into(), literal.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct SetFunctionFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetFunctionFn<'ctx> {
    const NAME: &'static str = "set_function";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let func_type = var_type
            .fn_type(&[var_type.ptr_type(AddressSpace::from(0)).into()], false)
            .ptr_type(AddressSpace::from(0));
        let u32_type = inkwell_context.context.i32_type();
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), func_type.into(), u32_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        func: &Function<'ctx, T>,
    ) {
        let args_num = compiler
            .inkwell_context
            .context
            .i32_type()
            .const_int(func.arg_names.len().try_into().unwrap(), false);
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[
                val.value.into(),
                func.function.as_global_value().as_pointer_value().into(),
                args_num.into(),
            ],
            "",
        );
    }
}

#[derive(Clone)]
pub struct SetVariableFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetVariableFn<'ctx> {
    const NAME: &'static str = "set_variable";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), var_type.into()], false);
        let func =
            inkwell_context
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
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[val1.value.into(), val2.value.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct GetBooleanFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> GetBooleanFn<'ctx> {
    const NAME: &'static str = "get_boolean";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .i8_type()
            .fn_type(&[var_type.into()], false);
        let func =
            inkwell_context
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
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into()], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value()
    }
}
