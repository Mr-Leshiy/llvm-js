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
        let func = compiler
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
        let func = compiler
            .module
            .add_function("variable_assert_eq", function_type, Some(Linkage::External));
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
