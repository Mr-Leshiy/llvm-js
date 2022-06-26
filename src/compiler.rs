use crate::llvm_ast::VariableName;
use inkwell::{builder::Builder, context::Context, module::Module, values::PointerValue};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable identifier {0}")]
    UndefinedVariable(VariableName),
    #[error("Variable with this identifier {0} already declared")]
    AlreadyDeclaredVariable(VariableName),
    #[error("Invalid compiled module, {0}")]
    InvalidModule(String),
    #[error("Cannot write module, {0}")]
    CannotWriteModule(String),
}

pub trait Compile {
    fn compile<'ctx>(self, compiler: &mut Compiler<'ctx>) -> Result<(), Error>;
}

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    pub variables: HashMap<VariableName, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            variables: HashMap::new(),
        }
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
    use inkwell::{context::Context, AddressSpace};

    #[test]
    fn tmp_test() {
        let context = Context::create();
        let module = context.create_module("module");
        let builder = context.create_builder();

        let function_type = context.void_type().fn_type(
            &[context.f64_type().ptr_type(AddressSpace::Generic).into()],
            false,
        );
        let function = module.add_function("main", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        let arg_1 = function.get_first_param().unwrap().into_pointer_value();

        builder.position_at_end(basic_block);

        let pointer_1 = builder.build_alloca(context.f64_type(), "a1");
        let pointer_2 = builder.build_alloca(context.f64_type(), "a2");
        builder.build_store(pointer_1, context.f64_type().const_float(64_f64));
        let value = builder.build_load(arg_1, "load");
        builder.build_store(pointer_2, value);

        builder.build_return(None);

        dbg!(module.print_to_string());
    }
}
