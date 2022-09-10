use super::{Compiler, PredefineFunctionName};
use crate::Variable;
use inkwell::{
    module::Linkage,
    values::{FunctionValue, PointerValue},
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
        let ret_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = ret_type.fn_type(&[], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));

        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>) -> PointerValue<'ctx> {
        compiler
            .builder
            .build_call(self.func, &[], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value()
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
        let self_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let number_type = compiler.context.f64_type();

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[self_type.into(), number_type.into()], false);
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
        let self_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let boolean_type = compiler.context.bool_type();

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[self_type.into(), boolean_type.into()], false);
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
                    .const_int(literal.then_some(1_u64).unwrap_or(0_u64), false)
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
        let self_type = compiler.variable_type.ptr_type(AddressSpace::Generic);
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[self_type.into(), string_type.into()], false);
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
        let self_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[self_type.into(), self_type.into()], false);
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
pub struct PrintFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for PrintFn<'ctx> {
    const NAME: &'static str = "print";
}

impl<'ctx> PrintFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let self_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[self_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}
