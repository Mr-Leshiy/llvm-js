use super::Compiler;
use inkwell::{types::StructType, AddressSpace};

pub struct DynamicType<'ctx> {
    struct_type: StructType<'ctx>,
}

impl<'ctx> DynamicType<'ctx> {
    pub fn declare(compiler: &mut Compiler<'ctx>) -> Self {
        let number_type = compiler.context.f64_type();
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);

        let struct_type = compiler
            .context
            .struct_type(&[number_type.into(), string_type.into()], false);

        Self { struct_type }
    }

    pub fn assign_number(&self, compiler: &mut Compiler<'ctx>) {}
}
