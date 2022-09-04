use super::ValueType;
use crate::Compiler;
use inkwell::{
    types::IntType,
    values::{IntValue, PointerValue},
};

pub struct Boolean<'ctx> {
    value: PointerValue<'ctx>,
}

impl<'ctx> ValueType<'ctx> for Boolean<'ctx> {
    fn new(value: PointerValue<'ctx>) -> Self {
        Self { value }
    }
}

impl<'ctx> Boolean<'ctx> {
    pub(crate) fn get_type<T>(compiler: &Compiler<'ctx, T>) -> IntType<'ctx> {
        compiler.context.bool_type()
    }

    pub(crate) fn load_value<T>(&self, compiler: &Compiler<'ctx, T>) -> IntValue<'ctx> {
        compiler.builder.build_load(self.value, "").into_int_value()
    }

    pub(crate) fn assign_literal<T>(&self, compiler: &Compiler<'ctx, T>, literal: bool) {
        let literal = compiler
            .context
            .bool_type()
            .const_int(literal.then_some(1_u64).unwrap_or(0_u64), false);
        compiler.builder.build_store(self.value, literal);
    }

    pub(crate) fn assign_variable<T>(&self, compiler: &Compiler<'ctx, T>, var: &Self) {
        let var = compiler.builder.build_load(var.value, "").into_int_value();
        compiler.builder.build_store(self.value, var);
    }
}
