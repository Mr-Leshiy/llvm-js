use super::Compiler;
use crate::{InkwellContext, Variable};
use inkwell::{module::Linkage, values::FunctionValue};

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
