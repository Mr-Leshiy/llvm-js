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
    #[allow(clippy::too_many_lines)]
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
                let ret = logical_and_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Or => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_or_fn = compiler.predefined_functions()?.logical_or();
                let ret = logical_or_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Eq => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_eq_fn = compiler.predefined_functions()?.logical_eq();
                let ret = logical_eq_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Ne => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_ne_fn = compiler.predefined_functions()?.logical_ne();
                let ret = logical_ne_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Gt => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_gt_fn = compiler.predefined_functions()?.logical_gt();
                let ret = logical_gt_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Ge => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_ge_fn = compiler.predefined_functions()?.logical_ge();
                let ret = logical_ge_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Lt => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_lt_fn = compiler.predefined_functions()?.logical_lt();
                let ret = logical_lt_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Le => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let logical_le_fn = compiler.predefined_functions()?.logical_le();
                let ret = logical_le_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            // Arithmetic
            BinaryExpType::Add => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_addition_fn = compiler.predefined_functions()?.arithmetic_addition();
                let ret = arithmetic_addition_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Sub => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_substraction_fn =
                    compiler.predefined_functions()?.arithmetic_substraction();
                let ret = arithmetic_substraction_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Mul => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_multiplication_fn =
                    compiler.predefined_functions()?.arithmetic_multiplication();
                let ret = arithmetic_multiplication_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
            BinaryExpType::Div => {
                let var1 = self.left.compile(compiler)?;
                let var2 = self.right.compile(compiler)?;
                let arithmetic_division_fn = compiler.predefined_functions()?.arithmetic_division();
                let ret = arithmetic_division_fn.call(compiler, &var1, &var2);
                if var1.is_tmp() {
                    var1.deallocate(compiler)?;
                }
                if var2.is_tmp() {
                    var2.deallocate(compiler)?;
                }
                Ok(ret)
            }
        }
    }
}
