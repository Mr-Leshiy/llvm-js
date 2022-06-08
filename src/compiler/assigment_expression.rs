use super::{literal::CompiledLiteral, Compile, Compiler, Error};
use crate::ast::{AssigmentExpression, RightAssigmentValue};
use inkwell::{module::Module, values::PointerValue};

impl<'ctx> Compile<'ctx> for AssigmentExpression {
    type Output = PointerValue<'ctx>;

    fn compile(
        &self,
        compiler: &mut Compiler<'ctx>,
        module: &Module<'ctx>,
    ) -> Result<Self::Output, Error> {
        match compiler.variables.get(&self.left).cloned() {
            Some(pointer) => match &self.right {
                RightAssigmentValue::Literal(literal) => {
                    match literal.compile(compiler, module)? {
                        CompiledLiteral::Number(number) => {
                            compiler.builder.build_store(pointer, number)
                        }
                        CompiledLiteral::String(string) => {
                            compiler.builder.build_store(pointer, string)
                        }
                    };

                    Ok(pointer)
                }
                RightAssigmentValue::Identifier(identifier) => {
                    match compiler.variables.get(identifier).cloned() {
                        Some(pointer) => {
                            compiler.variables.insert(self.left.clone(), pointer);
                            Ok(pointer)
                        }
                        None => Err(Error::UndefinedVariable(identifier.clone())),
                    }
                }
            },
            None => Err(Error::UndefinedVariable(self.left.clone())),
        }
    }
}
