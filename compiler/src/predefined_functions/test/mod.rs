use super::{Compiler, PredefineFunctionName};
use crate::{InkwellContext, Variable};
use inkwell::{module::Linkage, values::FunctionValue};

#[derive(Clone)]
pub struct AssertFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AssertFn<'ctx> {
    const NAME: &'static str = "assert";
}

impl<'ctx> AssertFn<'ctx> {
    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into()], false);
        let func = inkwell_context.module.add_function(
            "variable_assert",
            function_type,
            Some(Linkage::External),
        );
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
pub struct AssertEqFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AssertEqFn<'ctx> {
    const NAME: &'static str = "assert_eq";
}

impl<'ctx> AssertEqFn<'ctx> {
    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), var_type.into()], false);
        let func = inkwell_context.module.add_function(
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
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[val1.value.into(), val2.value.into()],
            "",
        );
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
