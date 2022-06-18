use super::{literal::CompiledLiteral, Compile, Compiler, Error};
use crate::ast::{RightAssigmentValue, VariableDeclaration};
use inkwell::{module::Module, values::PointerValue};

impl<'ctx> Compile<'ctx> for VariableDeclaration {
    type Output = PointerValue<'ctx>;

    fn compile(
        self,
        compiler: &mut Compiler<'ctx>,
        module: &Module<'ctx>,
    ) -> Result<Self::Output, Error> {
        match self.0.right {
            RightAssigmentValue::Literal(literal) => match literal.compile(compiler, module)? {
                CompiledLiteral::Number(number) => {
                    let pointer = compiler
                        .builder
                        .build_alloca(compiler.context.f64_type(), self.0.left.name.as_str());
                    compiler
                        .variables
                        .insert(self.0.left.clone(), pointer)
                        .map_err(|_| Error::AlreadyDeclaredVariable(self.0.left))?;
                    compiler.builder.build_store(pointer, number);
                    Ok(pointer)
                }
                CompiledLiteral::String(string) => {
                    let pointer = compiler
                        .builder
                        .build_alloca(string.get_type(), self.0.left.name.as_str());

                    compiler
                        .variables
                        .insert(self.0.left.clone(), pointer)
                        .map_err(|_| Error::AlreadyDeclaredVariable(self.0.left))?;

                    compiler.builder.build_store(pointer, string);
                    Ok(pointer)
                }
            },
            RightAssigmentValue::Identifier(identifier) => {
                match compiler.variables.get(&identifier).cloned() {
                    Some(pointer) => {
                        compiler
                            .variables
                            .insert(self.0.left.clone(), pointer)
                            .map_err(|_| Error::AlreadyDeclaredVariable(self.0.left))?;
                        Ok(pointer)
                    }
                    None => Err(Error::UndefinedVariable(identifier)),
                }
            }
        }
    }
}
