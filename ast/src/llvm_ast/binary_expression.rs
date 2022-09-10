use super::{Identifier, VariableExpression};
use compiler::{
    arithmetic_operations::{arithmetic_add, arithmetic_div, arithmetic_mul, arithmetic_sub},
    Compiler, Function, Variable,
};

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
    SEq,
    SNe,
    // Arithmetic
    Add,
    Sub,
    Div,
    Mul,
}

impl BinaryExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        match self.exp_type {
            // Logical
            BinaryExpType::And => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                let logical_and_fn = compiler.predefined_functions().get_logical_and()?;
                Ok(logical_and_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Or => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                let logical_or_fn = compiler.predefined_functions().get_logical_and()?;
                Ok(logical_or_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Eq => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                let logical_eq_fn = compiler.predefined_functions().get_logical_eq()?;
                Ok(logical_eq_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::Ne => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                let logical_ne_fn = compiler.predefined_functions().get_logical_ne()?;
                Ok(logical_ne_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::SEq => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                let logical_seq_fn = compiler.predefined_functions().get_logical_seq()?;
                Ok(logical_seq_fn.call(compiler, &var1, &var2))
            }
            BinaryExpType::SNe => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                let logical_sne_fn = compiler.predefined_functions().get_logical_sne()?;
                Ok(logical_sne_fn.call(compiler, &var1, &var2))
            }
            // Arithmetic
            BinaryExpType::Add => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                arithmetic_add(compiler, cur_function, var1, var2)
            }
            BinaryExpType::Sub => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                arithmetic_sub(compiler, cur_function, var1, var2)
            }
            BinaryExpType::Div => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                arithmetic_div(compiler, cur_function, var1, var2)
            }
            BinaryExpType::Mul => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                arithmetic_mul(compiler, cur_function, var1, var2)
            }
        }
    }
}
