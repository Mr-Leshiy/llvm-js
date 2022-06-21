use crate::{
    lexer::{self, CharReader},
    parser,
    parser::Parser,
    precompiler::{self, Precompile, Precompiler},
};
pub use assigment_expression::AssigmentExpression;
pub use block_statement::BlockStatement;
pub use expression::Expression;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
pub use literal::Literal;
pub use program::Program;
pub use right_assignment_value::RightAssigmentValue;
use std::io::Read;
pub use variable_declaration::VariableDeclaration;

mod assigment_expression;
mod block_statement;
mod expression;
mod function_declaration;
mod identifier;
mod literal;
mod program;
mod right_assignment_value;
mod variable_declaration;

/// Module
pub struct Module {
    pub name: String,
    pub program: Program,
}

impl Module {
    pub fn new<R: Read>(name: String, input: R) -> Result<Self, parser::Error> {
        let mut reader = CharReader::new(input);
        let program = Program::parse(lexer::get_token(&mut reader)?, &mut reader)?;
        Ok(Self { name, program })
    }

    pub fn precompile(self) -> Result<Precompiler, precompiler::Error> {
        let mut precompiler = Precompiler::new();
        self.program.precompile(&mut precompiler)?;
        Ok(precompiler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        AssigmentExpression, BlockStatement, Expression, FunctionDeclaration, Identifier, Literal,
        RightAssigmentValue, VariableDeclaration,
    };

    #[test]
    fn parse_module_from_file_test() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let module = Module::new("".to_string(), file).unwrap();
        let program = module.program;

        assert_eq!(
            program.body,
            vec![
                Expression::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier {
                        name: "foo".to_string()
                    },
                    args: vec![
                        Identifier {
                            name: "arg1".to_string()
                        },
                        Identifier {
                            name: "arg2".to_string()
                        }
                    ],
                    body: BlockStatement { body: vec![] }
                }),
                Expression::BlockStatement(BlockStatement {
                    body: vec![
                        Expression::VariableDeclaration(VariableDeclaration(AssigmentExpression {
                            left: Identifier {
                                name: "a".to_string()
                            },
                            right: RightAssigmentValue::Literal(Literal::Number(5_f64))
                        })),
                        Expression::VariableDeclaration(VariableDeclaration(AssigmentExpression {
                            left: Identifier {
                                name: "b".to_string()
                            },
                            right: RightAssigmentValue::Literal(Literal::Number(6_f64))
                        })),
                        Expression::BlockStatement(BlockStatement {
                            body: vec![
                                Expression::Assigment(AssigmentExpression {
                                    left: Identifier {
                                        name: "a".to_string()
                                    },
                                    right: RightAssigmentValue::Identifier(Identifier {
                                        name: "b".to_string()
                                    })
                                }),
                                Expression::Assigment(AssigmentExpression {
                                    left: Identifier {
                                        name: "b".to_string()
                                    },
                                    right: RightAssigmentValue::Literal(Literal::Number(7_f64))
                                }),
                                Expression::VariableDeclaration(VariableDeclaration(
                                    AssigmentExpression {
                                        left: Identifier {
                                            name: "c".to_string()
                                        },
                                        right: RightAssigmentValue::Literal(Literal::String(
                                            "hello".to_string()
                                        ))
                                    }
                                ))
                            ]
                        })
                    ]
                })
            ]
        );
    }
}
