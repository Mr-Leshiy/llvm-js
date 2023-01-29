use super::Compiler;
use crate::{Function, InkwellContext, Variable};
use inkwell::{
    module::Linkage,
    values::{FunctionValue, IntValue},
    AddressSpace,
};

#[derive(Clone)]
pub struct AllocateFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> AllocateFn<'ctx> {
    const NAME: &'static str = "allocate";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = var_type.fn_type(&[], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>) -> Variable<'ctx> {
        let value = compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable {
            value,
            is_tmp: false,
        }
    }
}

#[derive(Clone)]
pub struct DeallocateFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> DeallocateFn<'ctx> {
    const NAME: &'static str = "deallocate";

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
pub struct SetObjectFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetObjectFn<'ctx> {
    const NAME: &'static str = "set_object";

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
pub struct SetArrayFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> SetArrayFn<'ctx> {
    const NAME: &'static str = "set_array";

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

#[derive(Clone)]
pub struct FunctionCallFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> FunctionCallFn<'ctx> {
    const NAME: &'static str = "function_call";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let u32_type = inkwell_context.context.i32_type();
        let function_type = var_type.fn_type(
            &[
                var_type.into(),
                var_type.ptr_type(AddressSpace::from(0)).into(),
                u32_type.into(),
            ],
            false,
        );
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
        args: &[Variable<'ctx>],
    ) -> Variable<'ctx> {
        let var_type = compiler.inkwell_context.variable_type;

        let array = compiler
            .inkwell_context
            .builder
            .build_alloca(var_type.array_type(args.len().try_into().unwrap()), "");
        let args_len = compiler
            .inkwell_context
            .context
            .i32_type()
            .const_int(args.len().try_into().unwrap(), false);

        for (i, arg) in args.iter().enumerate() {
            unsafe {
                let ptr = compiler.inkwell_context.builder.build_gep(
                    array,
                    &[
                        compiler
                            .inkwell_context
                            .context
                            .i32_type()
                            .const_int(0, false),
                        compiler
                            .inkwell_context
                            .context
                            .i32_type()
                            .const_int(i.try_into().unwrap(), false),
                    ],
                    "",
                );
                let ptr = compiler
                    .inkwell_context
                    .builder
                    .build_bitcast(ptr, var_type.ptr_type(AddressSpace::from(0)), "")
                    .into_pointer_value();
                compiler.inkwell_context.builder.build_store(ptr, arg.value);
            }
        }

        let args = compiler.inkwell_context.builder.build_bitcast(
            array,
            var_type.ptr_type(AddressSpace::from(0)),
            "",
        );

        let value = compiler
            .inkwell_context
            .builder
            .build_call(
                self.func,
                &[val.value.into(), args.into(), args_len.into()],
                "",
            )
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable {
            value,
            is_tmp: true,
        }
    }
}

#[derive(Clone)]
pub struct AddPropertyByStrFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> AddPropertyByStrFn<'ctx> {
    const NAME: &'static str = "add_property_by_str";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let string_type = inkwell_context
            .context
            .i8_type()
            .ptr_type(AddressSpace::from(0));
        let function_type = inkwell_context.context.void_type().fn_type(
            &[var_type.into(), string_type.into(), var_type.into()],
            false,
        );
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
        key: &str,
        value: &Variable<'ctx>,
    ) {
        let key = compiler
            .inkwell_context
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[val.value.into(), key.into(), value.value.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct AddPropertyByVarFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> AddPropertyByVarFn<'ctx> {
    const NAME: &'static str = "add_property_by_var";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), var_type.into(), var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    #[allow(dead_code)]
    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &Variable<'ctx>,
        value: &Variable<'ctx>,
    ) {
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[val.value.into(), key.value.into(), value.value.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct GetPropertyByStrFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> GetPropertyByStrFn<'ctx> {
    const NAME: &'static str = "get_property_by_str";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let string_type = inkwell_context
            .context
            .i8_type()
            .ptr_type(AddressSpace::from(0));
        let boolean_type = inkwell_context.context.bool_type();
        let function_type = var_type.fn_type(
            &[var_type.into(), string_type.into(), boolean_type.into()],
            false,
        );
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
        key: &str,
        allocate: bool,
    ) -> Variable<'ctx> {
        let key = compiler
            .inkwell_context
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        let value = compiler
            .inkwell_context
            .builder
            .build_call(
                self.func,
                &[
                    val.value.into(),
                    key.into(),
                    compiler
                        .inkwell_context
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
        Variable {
            value,
            is_tmp: false,
        }
    }
}

#[derive(Clone)]
pub struct GetPropertyByVarFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> GetPropertyByVarFn<'ctx> {
    const NAME: &'static str = "get_property_by_var";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let boolean_type = inkwell_context.context.bool_type();
        let function_type = var_type.fn_type(
            &[var_type.into(), var_type.into(), boolean_type.into()],
            false,
        );
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
        key: &Variable<'ctx>,
        allocate: bool,
    ) -> Variable<'ctx> {
        let value = compiler
            .inkwell_context
            .builder
            .build_call(
                self.func,
                &[
                    val.value.into(),
                    key.value.into(),
                    compiler
                        .inkwell_context
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
        Variable {
            value,
            is_tmp: false,
        }
    }
}

#[derive(Clone)]
pub struct RemovePropertyFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> RemovePropertyFn<'ctx> {
    const NAME: &'static str = "remove_property";

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

    pub(crate) fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, key: &str) {
        let key = compiler
            .inkwell_context
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into(), key.into()], "");
    }
}
