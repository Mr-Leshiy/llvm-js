use super::{literal::CompiledLiteral, Compile, Compiler, Error};
use crate::ast::{RightAssigmentValue, VariableDeclaration};
use inkwell::values::PointerValue;

impl<'ctx> Compile<'ctx> for VariableDeclaration {
    type Output = PointerValue<'ctx>;
    fn compile(&self, compiler: &mut Compiler<'ctx>) -> Result<Self::Output, Error> {
        match &self.init {
            RightAssigmentValue::Identifier(identifier) => {
                match compiler.variables.get(identifier).cloned() {
                    Some(pointer) => {
                        compiler.variables.insert(self.id.clone(), pointer);

                        Ok(pointer)
                    }
                    None => Err(Error::UndefinedVariable(identifier.clone())),
                }
            }
            RightAssigmentValue::Literal(literal) => match literal.compile(compiler)? {
                CompiledLiteral::Number(number) => {
                    let pointer = compiler
                        .builder
                        .build_alloca(compiler.context.f64_type(), "var_1");

                    compiler.variables.insert(self.id.clone(), pointer);

                    compiler.builder.build_store(pointer, number);
                    Ok(pointer)
                }
                CompiledLiteral::String(string) => {
                    let pointer = compiler
                        .builder
                        .build_alloca(string.get_type(), self.id.name.as_str());

                    compiler.variables.insert(self.id.clone(), pointer);

                    compiler.builder.build_store(pointer, string);
                    Ok(pointer)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Identifier, Literal},
        compiler::Compiler,
    };
    use inkwell::{context::Context, values::AnyValue};

    #[test]
    fn variable_declaration_compile() {
        let ctx = Context::create();
        let mut compiler = Compiler::new(&ctx);

        let module = compiler.context.create_module("av");
        let void_type = compiler.context.void_type();
        let f32_type = compiler.context.f32_type();
        let i32_type = compiler.context.i32_type();
        let struct_type = compiler
            .context
            .struct_type(&[i32_type.into(), f32_type.into()], false);
        let array_type = i32_type.array_type(3);
        let fn_type = void_type.fn_type(&[], false);
        let fn_value = module.add_function("av_fn", fn_type, None);
        let entry = compiler.context.append_basic_block(fn_value, "entry1");

        compiler.builder.position_at_end(entry);

        let array_alloca = compiler.builder.build_alloca(array_type, "array_alloca");
    }
}
