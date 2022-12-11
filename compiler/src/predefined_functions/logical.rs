use super::{Compiler, PredefineFunctionName};
use crate::Variable;
use inkwell::{module::Linkage, values::FunctionValue, AddressSpace};

#[derive(Clone)]
pub struct LogicalNotFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalNotFn<'ctx> {
    const NAME: &'static str = "logical_not";
}

impl<'ctx> LogicalNotFn<'ctx> {
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
        Variable {
            value,
            is_tmp: true,
        }
    }
}

#[derive(Clone)]
pub struct LogicalAndFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalAndFn<'ctx> {
    const NAME: &'static str = "logical_and";
}

impl<'ctx> LogicalAndFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalOrFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalOrFn<'ctx> {
    const NAME: &'static str = "logical_or";
}

impl<'ctx> LogicalOrFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalEqFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalEqFn<'ctx> {
    const NAME: &'static str = "logical_eq";
}

impl<'ctx> LogicalEqFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalNeFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalNeFn<'ctx> {
    const NAME: &'static str = "logical_ne";
}

impl<'ctx> LogicalNeFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalSEqFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalSEqFn<'ctx> {
    const NAME: &'static str = "logical_seq";
}

impl<'ctx> LogicalSEqFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalSNeFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalSNeFn<'ctx> {
    const NAME: &'static str = "logical_sne";
}

impl<'ctx> LogicalSNeFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalGtFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalGtFn<'ctx> {
    const NAME: &'static str = "logical_gt";
}

impl<'ctx> LogicalGtFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalGeFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalGeFn<'ctx> {
    const NAME: &'static str = "logical_ge";
}

impl<'ctx> LogicalGeFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalLtFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalLtFn<'ctx> {
    const NAME: &'static str = "logical_lt";
}

impl<'ctx> LogicalLtFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
pub struct LogicalLeFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for LogicalLeFn<'ctx> {
    const NAME: &'static str = "logical_le";
}

impl<'ctx> LogicalLeFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = compiler.variable_type.ptr_type(AddressSpace::Generic);

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
