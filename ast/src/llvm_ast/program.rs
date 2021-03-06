use super::{expression::Expression, FunctionDeclaration, Identifier};
use compiler::{self, Compiler, Function};

pub struct Program {
    pub functions: Vec<FunctionDeclaration>,
    pub body: Vec<Expression>,
}

impl Program {
    pub fn compile(
        self,
        compiler: &mut Compiler<Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        for func in self.functions {
            func.compile(compiler)?;
        }

        // define main function
        let mut function = Function::new(compiler, "main", vec![]);
        function.generate_body(compiler, self.body)
    }
}
