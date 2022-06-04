use super::{Compile, CompileResult, Compiler, Error};
use crate::ast::Literal;
use inkwell::values::{AnyValue, FloatValue, VectorValue};

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
    fn compile(&self, compiler: &mut Compiler<'ctx>) -> Result<Self::Output, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Compiler;
    use inkwell::context::Context;

    #[test]
    fn literal_number_compile() {
        let ctx = Context::create();
        let mut compiler = Compiler::new(&ctx);

        let value = Literal::Number(123_f64).compile(&mut compiler).unwrap();
        assert_eq!(value.to_string(), "double 1.230000e+02");

        let value = Literal::Number(156_f64).compile(&mut compiler).unwrap();
        assert_eq!(value.to_string(), "double 1.560000e+02");
    }

    #[test]
    fn literal_string_compile() {
        let ctx = Context::create();
        let mut compiler = Compiler::new(&ctx);

        let value = Literal::String("hello".to_string())
            .compile(&mut compiler)
            .unwrap();
        assert_eq!(value.to_string(), "[5 x i8] c\"hello\"");

        let value = Literal::String("world".to_string())
            .compile(&mut compiler)
            .unwrap();
        assert_eq!(value.to_string(), "[5 x i8] c\"world\"");
    }
}
