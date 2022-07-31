use super::{Expression, Identifier};
use compiler::{self, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: Vec<Expression>,
}

impl FunctionDeclaration {
    pub fn compile(
        self,
        compiler: &mut Compiler<Identifier>,
    ) -> Result<(), compiler::Error<Identifier>> {
        let mut function = Function::new(
            compiler,
            &String::from(self.name.clone()),
            self.args.into_iter().map(|arg| arg.into()).collect(),
        );
        function.generate_body(compiler, self.body)?;

        compiler.insert_function(self.name.into(), function)
    }
}
