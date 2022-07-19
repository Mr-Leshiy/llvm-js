use crate::{
    llvm_ast,
    precompiler::{self, Precompile, Precompiler},
};
pub use block_statement::BlockStatement;
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
use lexer::{Parser, TokenReader};
pub use literal::Literal;
pub use program::Program;
pub use right_assignment_value::RightAssigmentValue;
use std::io::Read;
pub use variable_assigment::VariableAssigment;
pub use variable_declaration::VariableDeclaration;

mod block_statement;
mod expression;
mod function_call;
mod function_declaration;
mod identifier;
mod literal;
mod program;
mod right_assignment_value;
mod variable_assigment;
mod variable_declaration;

/// Module
pub struct Module {
    pub name: String,
    pub program: Program,
}

impl Module {
    pub fn new<R: Read>(name: String, input: R) -> Result<Self, lexer::Error> {
        let mut reader = TokenReader::new(input);
        let program = Program::parse(reader.next_token()?, &mut reader)?;
        Ok(Self { name, program })
    }

    pub fn precompile<Iter>(
        self,
        predefined_functions: Iter,
    ) -> Result<llvm_ast::Module, precompiler::Error>
    where
        Iter: Iterator<Item = Identifier>,
    {
        let mut precompiler = Precompiler::new(predefined_functions);

        let mut body = Vec::new();
        for expr in self.program.body {
            expr.precompile(&mut precompiler)?
                .into_iter()
                .for_each(|expr| body.push(expr));
        }

        Ok(llvm_ast::Module {
            name: self.name,
            program: llvm_ast::Program {
                functions: precompiler.function_declarations,
                body,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{
        BlockStatement, Expression, FunctionDeclaration, Literal, RightAssigmentValue,
        VariableAssigment, VariableDeclaration,
    };

    #[test]
    fn parse_module_from_file_test() {
        let file = std::fs::File::open("../test_scripts/basic.js").unwrap();
        let module = Module::new("".to_string(), file).unwrap();
        let program = module.program;

        assert_eq!(
            program.body,
            vec![
                Expression::FunctionDeclaration(FunctionDeclaration {
                    name: "foo".to_string().into(),
                    args: vec!["arg1".to_string().into(), "arg2".to_string().into()],
                    body: BlockStatement {
                        body: vec![Expression::VariableAssigment(VariableAssigment {
                            left: "arg1".to_string().into(),
                            right: RightAssigmentValue::Literal(Literal::Number(12_f64)),
                        })]
                    }
                }),
                Expression::BlockStatement(BlockStatement {
                    body: vec![
                        Expression::VariableDeclaration(VariableDeclaration(VariableAssigment {
                            left: "a".to_string().into(),
                            right: RightAssigmentValue::Literal(Literal::Number(5_f64))
                        })),
                        Expression::VariableDeclaration(VariableDeclaration(VariableAssigment {
                            left: "b".to_string().into(),
                            right: RightAssigmentValue::Literal(Literal::Number(6_f64))
                        })),
                        Expression::FunctionCall(FunctionCall {
                            name: "foo".to_string().into(),
                            args: vec!["a".to_string().into(), "b".to_string().into()]
                        }),
                        Expression::BlockStatement(BlockStatement {
                            body: vec![
                                Expression::VariableAssigment(VariableAssigment {
                                    left: "a".to_string().into(),
                                    right: RightAssigmentValue::Identifier("b".to_string().into())
                                }),
                                Expression::VariableAssigment(VariableAssigment {
                                    left: "b".to_string().into(),
                                    right: RightAssigmentValue::Literal(Literal::Number(7_f64))
                                }),
                                Expression::VariableDeclaration(VariableDeclaration(
                                    VariableAssigment {
                                        left: "c".to_string().into(),
                                        right: RightAssigmentValue::Literal(Literal::String(
                                            "hello".to_string()
                                        ))
                                    }
                                ))
                            ]
                        }),
                        Expression::FunctionCall(FunctionCall {
                            name: "foo".to_string().into(),
                            args: vec!["a".to_string().into(), "b".to_string().into()]
                        })
                    ]
                })
            ]
        );
    }
}
