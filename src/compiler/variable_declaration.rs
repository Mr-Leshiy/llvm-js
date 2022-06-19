use super::{Compile, Compiler, Error};
use crate::ast::{Literal, RightAssigmentValue, VariableDeclaration};
use inkwell::module::Module;

impl<'ctx> Compile<'ctx> for VariableDeclaration {
    fn compile(self, compiler: &mut Compiler<'ctx>, _: &Module<'ctx>) -> Result<(), Error> {
        match self.0.right {
            RightAssigmentValue::Literal(literal) => match literal {
                Literal::Number(number) => {
                    let number = compiler.context.f64_type().const_float(number);
                    let pointer = compiler
                        .builder
                        .build_alloca(compiler.context.f64_type(), self.0.left.name.as_str());
                    compiler
                        .variables
                        .insert(self.0.left.clone(), pointer)
                        .map_err(|_| Error::AlreadyDeclaredVariable(self.0.left))?;
                    compiler.builder.build_store(pointer, number);
                    Ok(())
                }
                Literal::String(string) => {
                    let string = compiler.context.const_string(string.as_bytes(), false);
                    let pointer = compiler
                        .builder
                        .build_alloca(string.get_type(), self.0.left.name.as_str());

                    compiler
                        .variables
                        .insert(self.0.left.clone(), pointer)
                        .map_err(|_| Error::AlreadyDeclaredVariable(self.0.left))?;

                    compiler.builder.build_store(pointer, string);
                    Ok(())
                }
            },
            RightAssigmentValue::Identifier(identifier) => {
                match compiler.variables.get(&identifier).cloned() {
                    Some(pointer) => {
                        compiler
                            .variables
                            .insert(self.0.left.clone(), pointer)
                            .map_err(|_| Error::AlreadyDeclaredVariable(self.0.left))?;
                        Ok(())
                    }
                    None => Err(Error::UndefinedVariable(identifier)),
                }
            }
        }
    }
}
