use super::{Compiler, PredefineFunctionName};
use crate::types::String;
use inkwell::{
    module::Linkage,
    values::{FunctionValue, IntValue},
    AddressSpace,
};

#[derive(Clone)]
pub struct StrcmpFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for StrcmpFn<'ctx> {
    const NAME: &'static str = "strcmp";
}

impl<'ctx> StrcmpFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let function_type = compiler.context.i32_type().fn_type(
            &[
                compiler
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::Generic)
                    .into(),
                compiler
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::Generic)
                    .into(),
            ],
            false,
        );
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));

        Self { func }
    }

    pub fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        var1: String<'ctx>,
        var2: String<'ctx>,
    ) -> IntValue<'ctx> {
        compiler
            .builder
            .build_call(
                self.func,
                &[
                    var1.load_value(compiler).into(),
                    var2.load_value(compiler).into(),
                ],
                "",
            )
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value()
    }
}
