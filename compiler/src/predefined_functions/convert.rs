use super::{Compiler, PredefineFunctionName};
use crate::Variable;
use inkwell::{module::Linkage, values::FunctionValue, AddressSpace};

#[derive(Clone)]
pub struct ConvertToBooleanFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for ConvertToBooleanFn<'ctx> {
    const NAME: &'static str = "convert_to_boolean";
}

impl<'ctx> ConvertToBooleanFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = var_type.fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) -> Variable<'ctx> {
        let value = compiler
            .builder
            .build_call(self.func, &[val.value.into()], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable { value }
    }
}

#[derive(Clone)]
pub struct ConvertToNumberFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for ConvertToNumberFn<'ctx> {
    const NAME: &'static str = "convert_to_number";
}

impl<'ctx> ConvertToNumberFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = var_type.fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) -> Variable<'ctx> {
        let value = compiler
            .builder
            .build_call(self.func, &[val.value.into()], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable { value }
    }
}

#[derive(Clone)]
pub struct ConvertToStringFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for ConvertToStringFn<'ctx> {
    const NAME: &'static str = "convert_to_string";
}

impl<'ctx> ConvertToStringFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

        let function_type = var_type.fn_type(&[var_type.into()], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>) -> Variable<'ctx> {
        let value = compiler
            .builder
            .build_call(self.func, &[val.value.into()], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable { value }
    }
}