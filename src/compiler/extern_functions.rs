use super::{Compiler, Error};
use inkwell::{
    module::Linkage,
    types::AnyTypeEnum,
    values::{FloatValue, FunctionValue, GlobalValue, PointerValue},
    AddressSpace,
};

#[derive(Clone)]
pub struct PrintfFn<'ctx> {
    func: FunctionValue<'ctx>,

    p_f64_fmt: GlobalValue<'ctx>,
    p_str_fmt: GlobalValue<'ctx>,
}

impl<'ctx> PrintfFn<'ctx> {
    pub fn declare(compiler: &mut Compiler<'ctx>) -> Self {
        let s = compiler.context.const_string(b"%f\n", true);
        let p_f64_fmt = compiler.module.add_global(s.get_type(), None, "p_f64_fmt");
        p_f64_fmt.set_constant(true);
        p_f64_fmt.set_externally_initialized(false);
        p_f64_fmt.set_initializer(&s);

        let s = compiler.context.const_string(b"%s\n", true);
        let p_str_fmt = compiler.module.add_global(s.get_type(), None, "p_str_fmt");
        p_str_fmt.set_constant(true);
        p_str_fmt.set_externally_initialized(false);
        p_str_fmt.set_initializer(&s);

        let function_type = compiler.context.i32_type().fn_type(
            &[compiler
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .into()],
            true,
        );
        let func = compiler
            .module
            .add_function("printf", function_type, Some(Linkage::External));

        Self {
            func,
            p_f64_fmt,
            p_str_fmt,
        }
    }

    fn print_float(&self, compiler: &mut Compiler<'ctx>, val: FloatValue<'ctx>) {
        compiler.builder.build_call(
            self.func,
            &[
                compiler
                    .builder
                    .build_pointer_cast(
                        self.p_f64_fmt.as_pointer_value(),
                        compiler.context.i8_type().ptr_type(AddressSpace::Generic),
                        "cast",
                    )
                    .into(),
                val.into(),
            ],
            "call",
        );
    }

    fn print_string(&self, compiler: &mut Compiler<'ctx>, val: PointerValue<'ctx>) {
        compiler.builder.build_call(
            self.func,
            &[
                compiler
                    .builder
                    .build_pointer_cast(
                        self.p_str_fmt.as_pointer_value(),
                        compiler.context.i8_type().ptr_type(AddressSpace::Generic),
                        "cast",
                    )
                    .into(),
                val.into(),
            ],
            "call",
        );
    }

    pub fn print(
        &self,
        compiler: &mut Compiler<'ctx>,
        val: PointerValue<'ctx>,
    ) -> Result<(), Error> {
        match val.get_type().get_element_type() {
            AnyTypeEnum::FloatType(_) => {
                let value = compiler.builder.build_load(val, "");
                self.print_float(compiler, value.into_float_value());
                Ok(())
            }
            AnyTypeEnum::ArrayType(_) => {
                self.print_string(compiler, val);
                Ok(())
            }
            _ => Err(Error::InvalidType),
        }
    }
}
