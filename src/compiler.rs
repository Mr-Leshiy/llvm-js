use crate::llvm_ast::VariableName;
use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    values::{FloatValue, FunctionValue, GlobalValue, PointerValue},
    AddressSpace,
};
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
    fn compile(self, compiler: &mut Compiler) -> Result<(), Error>;
}

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    pub variables: HashMap<VariableName, PointerValue<'ctx>>,

    pub printf: Option<FunctionValue<'ctx>>,

    pub p_f64_fmt: Option<GlobalValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            variables: HashMap::new(),
            printf: None,
            p_f64_fmt: None,
        }
    }

    pub fn declare_prinf(&mut self) {
        let s1 = self.context.const_string(b"%f\n", true);
        let global1 = self.module.add_global(s1.get_type(), None, "p_f64_fmt");
        global1.set_constant(true);
        global1.set_externally_initialized(false);
        global1.set_initializer(&s1);

        self.p_f64_fmt = Some(global1);

        let function_type = self.context.i32_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .into()],
            true,
        );
        let printf_function =
            self.module
                .add_function("printf", function_type, Some(Linkage::External));
        self.printf = Some(printf_function);
    }

    pub fn print_number(&self, val: FloatValue<'ctx>) {
        self.p_f64_fmt.unwrap().get_unnamed_address();
        self.builder.build_call(
            self.printf.unwrap(),
            &[
                self.builder
                    .build_pointer_cast(
                        self.p_f64_fmt.unwrap().as_pointer_value(),
                        self.context.i8_type().ptr_type(AddressSpace::Generic),
                        "cast",
                    )
                    .into(),
                val.into(),
            ],
            "call",
        );
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
