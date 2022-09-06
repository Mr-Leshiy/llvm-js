use super::{Compiler, PredefineFunctionName};
use inkwell::{module::Linkage, values::FunctionValue};

#[derive(Clone)]
pub struct AbortFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AbortFn<'ctx> {
    const NAME: &'static str = "abort";
}

impl<'ctx> AbortFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let function_type = compiler.context.void_type().fn_type(&[], false);
        let func = compiler
            .module
            .add_function("abort", function_type, Some(Linkage::External));

        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>) {
        compiler.builder.build_call(self.func, &[], "");
    }
}
