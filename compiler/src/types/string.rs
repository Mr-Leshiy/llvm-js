use super::ValueType;
use crate::Compiler;
use inkwell::{types::PointerType, values::PointerValue, AddressSpace};

pub struct String<'ctx> {
    value: PointerValue<'ctx>,
}

impl<'ctx> ValueType<'ctx> for String<'ctx> {
    fn new(value: PointerValue<'ctx>) -> Self {
        Self { value }
    }
}

impl<'ctx> String<'ctx> {
    pub(crate) fn get_type<T>(compiler: &Compiler<'ctx, T>) -> PointerType<'ctx> {
        compiler.context.i8_type().ptr_type(AddressSpace::Generic)
    }

    pub(crate) fn load_value<T>(&self, compiler: &Compiler<'ctx, T>) -> PointerValue<'ctx> {
        compiler
            .builder
            .build_load(self.value, "")
            .into_pointer_value()
    }

    pub(crate) fn assign_literal<T>(&self, compiler: &Compiler<'ctx, T>, literal: &str) {
        let string = compiler.context.const_string(literal.as_bytes(), true);
        let tmp_value = compiler.builder.build_alloca(string.get_type(), "");
        compiler.builder.build_store(tmp_value, string);
        let tmp_value = compiler
            .builder
            .build_bitcast(
                tmp_value,
                compiler.context.i8_type().ptr_type(AddressSpace::Generic),
                "",
            )
            .into_pointer_value();
        compiler.builder.build_store(self.value, tmp_value);
    }

    pub(crate) fn assign_variable<T>(&self, compiler: &Compiler<'ctx, T>, var: &Self) {
        let var = compiler
            .builder
            .build_load(var.value, "")
            .into_pointer_value();
        compiler.builder.build_store(self.value, var);
    }
}
