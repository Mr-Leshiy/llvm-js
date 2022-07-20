pub use function::Function;
use printf::PrintfFn;
use std::{collections::HashMap, io::Write, ops::Deref};
use thiserror::Error;
pub use variable::Variable;

mod function;
mod printf;
mod variable;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(String),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(String),
    #[error("Undefined function identifier {0}")]
    UndefinedFunction(String),
    #[error("Function with this identifier {0} already declared")]
    AlreadyDeclaredFunction(String),
    #[error("Not enough arguments")]
    NotEnoughArguments,
    #[error("Invalid compiled module, {0}")]
    InvalidModule(String),
    #[error("Cannot write module, {0}")]
    CannotWriteModule(String),
    #[error("Undeclared function: {0}")]
    UndeclaredFunction(String),
}

pub trait Compile {
    fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &Function<'ctx>,
    ) -> Result<(), Error>;
}

pub struct Context(inkwell::context::Context);

impl Deref for Context {
    type Target = inkwell::context::Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Context {
    pub fn new() -> Self {
        Self(inkwell::context::Context::create())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: inkwell::module::Module<'ctx>,
    builder: inkwell::builder::Builder<'ctx>,

    variables: HashMap<String, Variable<'ctx>>,
    functions: HashMap<String, Function<'ctx>>,

    printf: Option<PrintfFn<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            printf: None,
        }
    }

    pub fn declare_prinf(&mut self) {
        self.printf = Some(PrintfFn::declare(self));
    }

    pub fn get_printf(&self) -> Result<PrintfFn<'ctx>, Error> {
        self.printf
            .clone()
            .ok_or_else(|| Error::UndeclaredFunction("printf".to_string()))
    }
}

impl<'ctx> Compiler<'ctx> {
    pub fn insert_function(&mut self, name: String, function: Function<'ctx>) -> Result<(), Error> {
        match self.functions.insert(name.clone(), function) {
            None => Ok(()),
            Some(_) => Err(Error::AlreadyDeclaredFunction(name)),
        }
    }

    pub fn get_variable(
        &mut self,
        name: String,
        cur_function: &Function<'ctx>,
    ) -> Result<Variable<'ctx>, Error> {
        // firstly look into the function arguments
        for (i, arg_name) in cur_function.args.iter().enumerate() {
            if name.eq(arg_name) {
                let arg = cur_function
                    .function
                    .get_params()
                    .get(i)
                    .expect("")
                    .into_pointer_value();
                return Ok(Variable { value: arg });
            }
        }

        self.variables
            .get(&name)
            .cloned()
            .ok_or(Error::UndefinedVariable(name))
    }

    pub fn insert_variable(&mut self, name: String, variable: Variable<'ctx>) -> Result<(), Error> {
        match self.variables.insert(name.clone(), variable) {
            None => Ok(()),
            Some(_) => Err(Error::AlreadyDeclaredVariable(name)),
        }
    }

    pub fn get_function(&self, name: String) -> Result<Function<'ctx>, Error> {
        self.functions
            .get(&name)
            .cloned()
            .ok_or(Error::UndefinedFunction(name))
    }

    pub fn write_result_into<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        self.verify()?;
        writer
            .write(self.module.print_to_string().to_bytes())
            .map_err(|e| Error::CannotWriteModule(e.to_string()))?;
        Ok(())
    }

    pub fn verify(&self) -> Result<(), Error> {
        self.module
            .verify()
            .map_err(|e| Error::InvalidModule(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use inkwell::context::Context;

    #[test]
    fn tmp_test() {
        let context = Context::create();
        let module = context.create_module("module");
        let builder = context.create_builder();

        let function_type = context.void_type().fn_type(&[], false);

        let function = module.add_function("main", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let pointer_1 = builder.build_alloca(context.f64_type(), "a1");
        let pointer_2 = builder.build_alloca(context.f64_type(), "a2");
        builder.build_store(pointer_1, context.f64_type().const_float(64_f64));
        let value = builder.build_load(pointer_1, "load");
        builder.build_store(pointer_2, value);

        builder.build_return(None);

        dbg!(module.print_to_string());
    }
}
