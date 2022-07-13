use self::dynamic_type::DynamicType;
use extern_functions::PrintfFn;
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{FunctionValue, PointerValue},
};
use std::collections::HashMap;
use thiserror::Error;

mod dynamic_type;
mod extern_functions;

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
    #[error("InvalidType")]
    InvalidType,
}

pub trait Compile {
    fn compile(self, compiler: &mut Compiler) -> Result<(), Error>;
}

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    pub variables: HashMap<String, PointerValue<'ctx>>,
    pub functions: HashMap<String, FunctionValue<'ctx>>,

    dynamic_type: Option<DynamicType<'ctx>>,

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
            dynamic_type: None,
            printf: None,
        }
    }

    pub fn declare_dynamic_type(&mut self) {
        self.dynamic_type = Some(DynamicType::declare(self));
    }

    pub fn declare_prinf(&mut self) {
        self.printf = Some(PrintfFn::declare(self));
    }

    pub fn get_printf(&self) -> Result<PrintfFn<'ctx>, Error> {
        self.printf
            .clone()
            .ok_or_else(|| Error::UndeclaredFunction("printf".to_string()))
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
