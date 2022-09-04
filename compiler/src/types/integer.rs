use super::ValueType;
use crate::Compiler;
use inkwell::{
    types::IntType,
    values::{IntValue, PointerValue},
};

pub struct IntegerI8<'ctx> {
    value: PointerValue<'ctx>,
}

impl<'ctx> ValueType<'ctx> for IntegerI8<'ctx> {
    fn new(value: PointerValue<'ctx>) -> Self {
        Self { value }
    }
}

impl<'ctx> IntegerI8<'ctx> {
    pub(crate) fn get_type<T>(compiler: &Compiler<'ctx, T>) -> IntType<'ctx> {
        compiler.context.i8_type()
    }

    pub(crate) fn load_value<T>(&self, compiler: &Compiler<'ctx, T>) -> IntValue<'ctx> {
        compiler.builder.build_load(self.value, "").into_int_value()
    }

    pub(crate) fn assign_literal<T>(&self, compiler: &Compiler<'ctx, T>, literal: u64) {
        let literal = compiler.context.i8_type().const_int(literal, false);
        compiler.builder.build_store(self.value, literal);
    }
}
