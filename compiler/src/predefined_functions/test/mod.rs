use super::{Compiler, PredefineFunctionName};
use crate::Variable;
use inkwell::{module::Linkage, values::FunctionValue, AddressSpace};

#[derive(Clone)]
pub struct AssertFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AssertFn<'ctx> {
    const NAME: &'static str = "assert";
}

impl<'ctx> AssertFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func =
            compiler
                .module
                .add_function("variable_assert", function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) {
        compiler
            .builder
            .build_call(self.func, &[val.value.into()], "");
    }
}

#[derive(Clone)]
pub struct AssertEqFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AssertEqFn<'ctx> {
    const NAME: &'static str = "assert_eq";
}

impl<'ctx> AssertEqFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[var_type.into(), var_type.into()], false);
        let func = compiler.module.add_function(
            "variable_assert_eq",
            function_type,
            Some(Linkage::External),
        );
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
pub struct GbVariablesCount<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for GbVariablesCount<'ctx> {
    const NAME: &'static str = "gb_variables_count";
}

impl<'ctx> GbVariablesCount<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = var_type.fn_type(&[], false);
        let func = compiler.module.add_function(
            "gb_variables_count",
            function_type,
            Some(Linkage::External),
        );
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
        Variable {
            value,
            is_tmp: false,
        }
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
