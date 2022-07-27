use super::{Expression, VariableName};
use compiler::{self, Compiler, Function};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionName {
    name: String,
    index: u32,
}

impl FunctionName {
    pub fn new(name: String, index: u32) -> Self {
        Self { name, index }
    }
}

impl From<FunctionName> for String {
    fn from(val: FunctionName) -> Self {
        if val.index == 0 {
            val.name
        } else {
            format!("{}{}", val.name, val.index)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: FunctionName,
    pub args: Vec<VariableName>,
    pub body: Vec<Expression>,
}

impl FunctionDeclaration {
    pub fn compile(self, compiler: &mut Compiler) -> Result<(), compiler::Error> {
        let mut function = Function::new(
            compiler,
            &String::from(self.name.clone()),
            self.args.into_iter().map(|arg| arg.into()).collect(),
        );
        function.generate_body(compiler, self.body)?;

        compiler.insert_function(self.name.into(), function)
    }
}
