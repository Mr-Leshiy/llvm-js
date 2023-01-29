use super::Compiler;
use crate::{InkwellContext, Variable};
use inkwell::{module::Linkage, values::FunctionValue};

#[derive(Clone)]
pub struct ArithmeticAdditionFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> ArithmeticAdditionFn<'ctx> {
    pub const NAME: &'static str = "arithmetic_addition";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val1.value.into(), val2.value.into()], "")
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
pub struct ArithmeticSubstractionFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> ArithmeticSubstractionFn<'ctx> {
    const NAME: &'static str = "arithmetic_substraction";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val1.value.into(), val2.value.into()], "")
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
pub struct ArithmeticMultiplicationFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> ArithmeticMultiplicationFn<'ctx> {
    const NAME: &'static str = "arithmetic_multiplication";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val1.value.into(), val2.value.into()], "")
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
pub struct ArithmeticDivisionFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> ArithmeticDivisionFn<'ctx> {
    const NAME: &'static str = "arithmetic_division";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val1.value.into(), val2.value.into()], "")
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
