use crate::{llvm_ast, Error, Precompiler};
pub use array_expression::ArrayExpression;
pub use binary_expression::{BinaryExpType, BinaryExpression};
pub use block_statement::BlockStatement;
pub use do_while_loop::DoWhileLoop;
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
pub use if_else_statement::IfElseStatement;
use lexer::{Token, TokenReader};
pub use member_expression::{MemberExpression, Property};
pub use object_expression::ObjectExpression;
use std::io::Read;
pub use unary_expression::{UnaryExpType, UnaryExpression};
pub use variable_assigment::VariableAssigment;
pub use variable_declaration::VariableDeclaration;
pub use variable_expression::VariableExpression;
pub use variable_value::VariableValue;
pub use while_loop::WhileLoop;

mod array_expression;
mod binary_expression;
mod block_statement;
mod do_while_loop;
mod expression;
mod function_call;
mod function_declaration;
mod identifier;
mod if_else_statement;
mod member_expression;
mod object_expression;
mod return_statement;
mod unary_expression;
mod variable_assigment;
mod variable_declaration;
mod variable_expression;
mod variable_value;
mod while_loop;

/// Module
pub struct Module {
    name: String,
    body: Vec<Expression>,
}

impl Module {
    pub fn new<R: Read>(name: String, input: R) -> Result<Self, Error> {
        let mut reader = TokenReader::new(input);

        let mut body = Vec::new();
        let mut cur_token = reader.next_token()?;

        loop {
            let expr = match cur_token {
                Token::Eof => break,
                cur_token => Expression::parse(cur_token, &mut reader)?,
            };

            cur_token = reader.next_token()?;
            body.push(expr);
        }
        Ok(Self { name, body })
    }

    pub fn precompile<Iter>(self, predefined_functions: Iter) -> Result<llvm_ast::Module, Error>
    where
        Iter: Iterator<Item = Identifier>,
    {
        let mut precompiler = Precompiler::new(predefined_functions);

        let mut body = Vec::new();
        for expr in self.body {
            expr.precompile(&mut precompiler)?
                .into_iter()
                .for_each(|expr| body.push(expr));
        }
        let vars = precompiler.remove_last_added_variables(precompiler.variables_len());
        for (var, index) in vars {
            body.push(llvm_ast::Expression::DeallocateExpression(
                llvm_ast::DeallocateExpression {
                    name: llvm_ast::Identifier::new(var.name, index),
                },
            ));
        }

        Ok(llvm_ast::Module::new(
            self.name,
            precompiler.function_declarations,
            body,
        ))
    }
}
