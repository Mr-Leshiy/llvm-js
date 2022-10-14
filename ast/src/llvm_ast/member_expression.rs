use super::Identifier;
use compiler::{Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub object: Identifier,
    pub property: Option<Box<MemberExpression>>,
}

impl MemberExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        Variable::new_variable(compiler, &cur_function.get_variable(self.object)?)
    }
}
