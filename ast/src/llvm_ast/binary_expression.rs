use super::{Identifier, VariableExpression};
use compiler::{
    logical_operations::{
        logical_and, logical_eq, logical_ne, logical_or, logical_seq, logical_sne,
    },
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
    And,
    Or,
    Eq,
    Ne,
    SEq,
    SNe,
}

impl BinaryExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        match self.exp_type {
            BinaryExpType::And => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                logical_and(compiler, cur_function, var1, var2)
            }
            BinaryExpType::Or => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                logical_or(compiler, cur_function, var1, var2)
            }
            BinaryExpType::Eq => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                logical_eq(compiler, cur_function, var1, var2)
            }
            BinaryExpType::Ne => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                logical_ne(compiler, cur_function, var1, var2)
            }
            BinaryExpType::SEq => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                logical_seq(compiler, cur_function, var1, var2)
            }
            BinaryExpType::SNe => {
                let var1 = self.left.compile(compiler, cur_function)?;
                let var2 = self.right.compile(compiler, cur_function)?;
                logical_sne(compiler, cur_function, var1, var2)
            }
        }
    }
}