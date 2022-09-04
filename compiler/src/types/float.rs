use super::ValueType;
use crate::Compiler;
use inkwell::{
    types::FloatType,
    values::{FloatValue, PointerValue},
};

pub struct Float<'ctx> {
    value: PointerValue<'ctx>,
}

impl<'ctx> ValueType<'ctx> for Float<'ctx> {
    fn new(value: PointerValue<'ctx>) -> Self {
        Self { value }
    }
}

impl<'ctx> Float<'ctx> {
    pub(crate) fn get_type<T>(compiler: &Compiler<'ctx, T>) -> FloatType<'ctx> {
        compiler.context.f64_type()
    }

    pub(crate) fn load_value<T>(&self, compiler: &Compiler<'ctx, T>) -> FloatValue<'ctx> {
        compiler
            .builder
            .build_load(self.value, "")
            .into_float_value()
    }

    pub(crate) fn assign_literal<T>(&self, compiler: &Compiler<'ctx, T>, literal: f64) {
        let literal = compiler.context.f64_type().const_float(literal);
        compiler.builder.build_store(self.value, literal);
    }

    pub(crate) fn assign_variable<T>(&self, compiler: &Compiler<'ctx, T>, var: &Self) {
        let var = compiler
            .builder
            .build_load(var.value, "")
            .into_float_value();
        compiler.builder.build_store(self.value, var);
    }
}
