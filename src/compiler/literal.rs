use super::{Compile, CompileResult, Compiler, Error};
use crate::ast::Literal;
use inkwell::{
    module::Module,
    values::{AnyValue, FloatValue, VectorValue},
};

#[derive(Debug)]
pub enum CompiledLiteral<'ctx> {
    Number(FloatValue<'ctx>),
    String(VectorValue<'ctx>),
}

impl<'ctx> CompileResult for CompiledLiteral<'ctx> {
    fn to_string(&self) -> String {
        match self {
            Self::Number(number) => number.print_to_string().to_string(),
            Self::String(string) => string.print_to_string().to_string(),
        }
    }
}

impl<'ctx> Compile<'ctx> for Literal {
    type Output = CompiledLiteral<'ctx>;

    fn compile(&self, compiler: &mut Compiler<'ctx>, _: &Module) -> Result<Self::Output, Error> {
        match self {
            Literal::Number(val) => Ok(CompiledLiteral::Number(
                compiler.context.f64_type().const_float(*val),
            )),
            Literal::String(val) => Ok(CompiledLiteral::String(
                compiler.context.const_string(val.as_bytes(), false),
            )),
        }
    }
}
