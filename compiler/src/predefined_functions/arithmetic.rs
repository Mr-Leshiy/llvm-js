use super::{Compiler, PredefineFunctionName};
use crate::Variable;
use inkwell::{module::Linkage, values::FunctionValue};

#[derive(Clone)]
pub struct ArithmeticAdditionFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for ArithmeticAdditionFn<'ctx> {
    const NAME: &'static str = "arithmetic_addition";
}

impl<'ctx> ArithmeticAdditionFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type;

        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
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

impl<'ctx> PredefineFunctionName for ArithmeticSubstractionFn<'ctx> {
    const NAME: &'static str = "arithmetic_substraction";
}

impl<'ctx> ArithmeticSubstractionFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type;

        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
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

impl<'ctx> PredefineFunctionName for ArithmeticMultiplicationFn<'ctx> {
    const NAME: &'static str = "arithmetic_multiplication";
}

impl<'ctx> ArithmeticMultiplicationFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type;

        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
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

impl<'ctx> PredefineFunctionName for ArithmeticDivisionFn<'ctx> {
    const NAME: &'static str = "arithmetic_division";
}

impl<'ctx> ArithmeticDivisionFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type;

        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
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
    ) -> Variable<'ctx> {
        let value = compiler
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
