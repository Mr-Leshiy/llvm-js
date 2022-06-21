use super::{Identifier, Literal, RightAssigmentValue};
use crate::{
    compiler::{self, Compile, Compiler},
    lexer::{self, CharReader, Token},
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use inkwell::module::Module;
use std::io::Read;

/// AssigmentExpression - Expression type for variable assigment, like "a = 4"
#[derive(Debug, PartialEq)]
pub struct AssigmentExpression {
    pub left: Identifier,
    pub right: RightAssigmentValue,
}

impl Parser for AssigmentExpression {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        let left = Identifier::parse(cur_token, reader)?;

        match lexer::get_token(reader)? {
            Token::Assign => {}
            token => return Err(parser::Error::UnexpectedToken(token)),
        }

        let right = RightAssigmentValue::parse(lexer::get_token(reader)?, reader)?;
        Ok(Self { left, right })
    }
}

impl Precompile for AssigmentExpression {
    fn precompile(&self, precompiler: &mut Precompiler) -> Result<(), precompiler::Error> {
        Ok(())
    }
}

impl<'ctx> Compile<'ctx> for AssigmentExpression {
    fn compile(
        self,
        compiler: &mut Compiler<'ctx>,
        _: &Module<'ctx>,
    ) -> Result<(), compiler::Error> {
        match compiler.variables.get(&self.left).cloned() {
            Some(pointer) => match self.right {
                RightAssigmentValue::Literal(literal) => {
                    match literal {
                        Literal::Number(number) => {
                            let number = compiler.context.f64_type().const_float(number);
                            compiler.builder.build_store(pointer, number)
                        }
                        Literal::String(string) => {
                            let string = compiler.context.const_string(string.as_bytes(), false);
                            compiler.builder.build_store(pointer, string)
                        }
                    };
                    Ok(())
                }
                RightAssigmentValue::Identifier(identifier) => {
                    match compiler.variables.get(&identifier).cloned() {
                        Some(pointer) => {
                            compiler
                                .variables
                                .update(self.left.clone(), pointer)
                                .unwrap();
                            Ok(())
                        }
                        None => Err(compiler::Error::UndefinedVariable(identifier)),
                    }
                }
            },
            None => Err(compiler::Error::UndefinedVariable(self.left.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Identifier, Literal, RightAssigmentValue};

    #[test]
    fn assigment_expression_test() {
        let mut reader = CharReader::new("name = 12;".as_bytes());
        assert_eq!(
            AssigmentExpression::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            AssigmentExpression {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            }
        );

        let mut reader = CharReader::new("name1 = name2;".as_bytes());
        assert_eq!(
            AssigmentExpression::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            AssigmentExpression {
                left: Identifier {
                    name: "name1".to_string()
                },
                right: RightAssigmentValue::Identifier(Identifier {
                    name: "name2".to_string()
                })
            }
        );
    }
}
