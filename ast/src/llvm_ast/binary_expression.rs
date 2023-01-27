use super::VariableExpression;
use crate::{Compiler, CompilerError};
use compiler::Variable;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: VariableExpression,
    pub right: VariableExpression,
    pub exp_type: BinaryExpType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryExpType {
    // Logical
    And,
    Or,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    // Arithmetic
    Add,
    Sub,
    Div,
    Mul,
}

impl BinaryExpression {
    pub fn binary_call<
        'ctx,
        F: FnOnce(
            &Compiler<'ctx>,
            &Variable<'ctx>,
            &Variable<'ctx>,
        ) -> Result<Variable<'ctx>, CompilerError>,
    >(
        self,
        compiler: &mut Compiler<'ctx>,
        call: F,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let var1 = self.left.compile(compiler)?;
        let var2 = self.right.compile(compiler)?;
        let ret = call(compiler, &var1, &var2)?;
        if var1.is_tmp() {
            var1.deallocate(compiler)?;
        }
        if var2.is_tmp() {
            var2.deallocate(compiler)?;
        }
        Ok(ret)
    }

    #[allow(clippy::too_many_lines)]
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        match self.exp_type {
            // Logical
            BinaryExpType::And => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_and()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Or => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_or()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Eq => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_eq()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Ne => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_ne()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Gt => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_gt()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Ge => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_ge()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Lt => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_lt()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Le => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .logical_le()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            // Arithmetic
            BinaryExpType::Add => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .arithmetic_addition()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Sub => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .arithmetic_substraction()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Mul => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .arithmetic_multiplication()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
            BinaryExpType::Div => {
                let ret = self.binary_call(compiler, |compiler, var1, var2| {
                    Ok(compiler
                        .predefined_functions()?
                        .arithmetic_division()
                        .call(compiler, var1, var2))
                })?;
                Ok(ret)
            }
        }
    }
}
