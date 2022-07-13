use super::{expression::Expression, FunctionDeclaration};
use compiler::{self, Compile, Compiler, Function};

pub struct Program {
    pub functions: Vec<FunctionDeclaration>,
    pub body: Vec<Expression>,
}

impl Compile for Program {
    fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        for func in self.functions {
            func.compile(compiler)?;
        }

        // define main function
        let function = Function::new(compiler, "main");
        function.generate_body(compiler, self.body)
    }
}
