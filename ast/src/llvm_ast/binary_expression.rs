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
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        match self.exp_type {
            // Logical
            BinaryExpType::And => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_and_fn = compiler.predefined_functions()?.logical_and();
                Ok(logical_and_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Or => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_or_fn = compiler.predefined_functions()?.logical_or();
                Ok(logical_or_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Eq => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_eq_fn = compiler.predefined_functions()?.logical_eq();
                Ok(logical_eq_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Ne => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_ne_fn = compiler.predefined_functions()?.logical_ne();
                Ok(logical_ne_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Gt => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_gt_fn = compiler.predefined_functions()?.logical_gt();
                Ok(logical_gt_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Ge => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_ge_fn = compiler.predefined_functions()?.logical_ge();
                Ok(logical_ge_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Lt => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_lt_fn = compiler.predefined_functions()?.logical_lt();
                Ok(logical_lt_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Le => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_le_fn = compiler.predefined_functions()?.logical_le();
                Ok(logical_le_fn.call(compiler, &var1, &var2))
            }
            // Arithmetic
            BinaryExpType::Add => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_addition_fn = compiler.predefined_functions()?.arithmetic_addition();
                Ok(arithmetic_addition_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Sub => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_substraction_fn =
                    compiler.predefined_functions()?.arithmetic_substraction();
                Ok(arithmetic_substraction_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Mul => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_multiplication_fn =
                    compiler.predefined_functions()?.arithmetic_multiplication();
                Ok(arithmetic_multiplication_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Div => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_division_fn = compiler.predefined_functions()?.arithmetic_division();
                Ok(arithmetic_division_fn.call(compiler, &var1, &var2))
            }
        }
    }
}
