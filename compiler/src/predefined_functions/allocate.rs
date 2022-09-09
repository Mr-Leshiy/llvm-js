use super::{Compiler, PredefineFunctionName};
use inkwell::{module::Linkage, values::FunctionValue, AddressSpace};

#[derive(Clone)]
pub struct AllocateFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName for AllocateFn<'ctx> {
    const NAME: &'static str = "allocate";
}

impl<'ctx> AllocateFn<'ctx> {
    pub(super) fn declare<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let ret_type = compiler
            .context
            .opaque_struct_type("js_variable_type")
            .ptr_type(AddressSpace::Generic);

        let function_type = ret_type.fn_type(&[], false);
        let func = compiler
            .module
            .add_function(Self::NAME, function_type, Some(Linkage::External));

        Self { func }
    }

    pub fn call<T>(&self, compiler: &Compiler<'ctx, T>) {
        compiler.builder.build_call(self.func, &[], "");
    }
}
