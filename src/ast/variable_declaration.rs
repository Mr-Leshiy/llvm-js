use super::{AssigmentExpression, Literal, RightAssigmentValue};
use crate::{
    compiler::{self, Compile, Compiler},
    lexer::{self, CharReader, Keyword, Token},
    parser::{self, Parser},
};
use inkwell::module::Module;
use std::io::Read;

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4"
#[derive(Debug, PartialEq)]
pub struct VariableDeclaration(pub AssigmentExpression);

impl Parser for VariableDeclaration {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Keyword(Keyword::Var) => Ok(Self(AssigmentExpression::parse(
                lexer::get_token(reader)?,
                reader,
            )?)),
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

impl<'ctx> Compile<'ctx> for VariableDeclaration {
    fn compile(
        self,
        compiler: &mut Compiler<'ctx>,
        _: &Module<'ctx>,
    ) -> Result<(), compiler::Error> {
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
                        .map_err(|_| compiler::Error::AlreadyDeclaredVariable(self.0.left))?;
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
                        .map_err(|_| compiler::Error::AlreadyDeclaredVariable(self.0.left))?;

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
                            .map_err(|_| compiler::Error::AlreadyDeclaredVariable(self.0.left))?;
                        Ok(())
                    }
                    None => Err(compiler::Error::UndefinedVariable(identifier)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Identifier, Literal, RightAssigmentValue};

    #[test]
    fn variable_declaration_test() {
        let mut reader = CharReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            VariableDeclaration(AssigmentExpression {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            })
        );

        let mut reader = CharReader::new("var name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            VariableDeclaration(AssigmentExpression {
                left: Identifier {
                    name: "name1".to_string()
                },
                right: RightAssigmentValue::Identifier(Identifier {
                    name: "name2".to_string()
                })
            })
        );
    }
}
