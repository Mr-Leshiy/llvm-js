use super::{Compiler, PredefineFunction, PredefineFunctionName};
use inkwell::{module::Linkage, values::FunctionValue};

#[derive(Clone)]
pub struct AbortFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunction<'ctx> for AbortFn<'ctx> {}

impl<'ctx> PredefineFunctionName<'ctx> for AbortFn<'ctx> {
    const NAME: &'static str = "abort";
}

impl<'ctx> AbortFn<'ctx> {
    pub(super) fn declare(compiler: &Compiler<'ctx>) -> Self {
        let function_type = compiler.context.void_type().fn_type(&[], false);
        let func = compiler
            .module
            .add_function("abort", function_type, Some(Linkage::External));

        Self { func }
    }
}

impl<'ctx> AbortFn<'ctx> {
    pub fn abort(&self, compiler: &Compiler<'ctx>) {
        compiler.builder.build_call(self.func, &[], "");
    }
}
