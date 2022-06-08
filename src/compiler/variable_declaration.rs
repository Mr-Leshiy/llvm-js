use super::{literal::CompiledLiteral, Compile, CompileResult, Compiler, Error};
use crate::ast::{RightAssigmentValue, VariableDeclaration};
use inkwell::values::{AnyValue, PointerValue};

impl<'ctx> CompileResult for PointerValue<'ctx> {
    fn to_string(&self) -> String {
        self.print_to_string().to_string()
    }
}

impl<'ctx> Compile<'ctx> for VariableDeclaration {
    type Output = PointerValue<'ctx>;

    fn compile(
        &self,
        compiler: &mut Compiler<'ctx>,
        module_name: &String,
    ) -> Result<Self::Output, Error> {
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
            RightAssigmentValue::Literal(literal) => {
                match literal.compile(compiler, module_name)? {
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
                }
            }
        }
    }
}
