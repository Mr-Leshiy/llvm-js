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
                        .build_alloca(compiler.context.f64_type(), self.id.name.as_str());

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
        compiler::{CompileResult, Compiler},
    };
    use inkwell::context::Context;

    #[test]
    fn variable_declaration_compile() {
        let ctx = Context::create();
        let mut compiler = Compiler::new(&ctx);

        let module = compiler.context.create_module("test_module");
        let func = module.add_function(
            "test",
            compiler.context.void_type().fn_type(&[], false),
            None,
        );
        let block = compiler.context.append_basic_block(func, "entry");
        compiler.builder.position_at_end(block);

        let res = VariableDeclaration {
            id: Identifier {
                name: "var_1".to_string(),
            },
            init: RightAssigmentValue::Literal(Literal::Number(64_f64)),
        }
        .compile(&mut compiler)
        .unwrap();

        assert_eq!(module.print_to_string().to_string(), "");
    }
}
