use super::{BinaryExpression, FunctionCall, MemberExpression, UnaryExpression, VariableValue};
use crate::{Compiler, CompilerError};
use compiler::Variable;

#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    VariableValue(VariableValue),
    UnaryExpression(Box<UnaryExpression>),
    BinaryExpression(Box<BinaryExpression>),
    MemberExpression(Box<MemberExpression>),
    FunctionCall(FunctionCall),
}

impl VariableExpression {
    pub fn compile_get_variable<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        match self {
            Self::VariableValue(value) => value.compile(compiler),
            Self::UnaryExpression(expr) => expr.compile(compiler),
            Self::BinaryExpression(expr) => expr.compile(compiler),
            Self::MemberExpression(expr) => expr.compile_get_variable(compiler),
            Self::FunctionCall(function_call) => function_call.compile(compiler),
        }
    }

    pub fn compile_update_variable<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        new_value: &Variable<'ctx>,
    ) -> Result<(), CompilerError> {
        match self {
            Self::VariableValue(value) => {
                let value = value.compile(compiler)?;
                value.assign_variable(compiler, new_value);
                if value.is_tmp() {
                    value.deallocate(compiler);
                }
            }
            Self::UnaryExpression(expr) => {
                let value = expr.compile(compiler)?;
                value.assign_variable(compiler, new_value);
                if value.is_tmp() {
                    value.deallocate(compiler);
                }
            }
            Self::BinaryExpression(expr) => {
                let value = expr.compile(compiler)?;
                value.assign_variable(compiler, new_value);
                if value.is_tmp() {
                    value.deallocate(compiler);
                }
            }
            Self::FunctionCall(function_call) => {
                let value = function_call.compile(compiler)?;
                value.assign_variable(compiler, new_value);
                if value.is_tmp() {
                    value.deallocate(compiler);
                }
            }
            Self::MemberExpression(expr) => {
                expr.compile_update_variable(compiler, new_value)?;
            }
        }
        Ok(())
    }
}
