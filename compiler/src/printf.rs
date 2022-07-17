use super::Compiler;
use crate::{variable::Field, Error, Function};
use inkwell::{
    module::Linkage,
    values::{FunctionValue, GlobalValue},
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

    pub fn print(
        &self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &Function<'ctx>,
        args_names: Vec<String>,
    ) -> Result<(), Error> {
        let arg_name = args_names
            .into_iter()
            .next()
            .ok_or(Error::NotEnoughArguments)?;
        let variable = compiler
            .variables
            .get(&arg_name)
            .cloned()
            .ok_or(Error::UndefinedVariable(arg_name))?;

        let number_case_f = |compiler: &mut Compiler<'ctx>| {
            let number_field = variable.get_field(compiler, Field::Number);
            let number_field = compiler
                .builder
                .build_load(number_field, "")
                .into_float_value();
            compiler.builder.build_call(
                self.func,
                &[
                    compiler
                        .builder
                        .build_pointer_cast(
                            self.p_f64_fmt.as_pointer_value(),
                            compiler.context.i8_type().ptr_type(AddressSpace::Generic),
                            "",
                        )
                        .into(),
                    number_field.into(),
                ],
                "",
            );
        };
        let string_case_f = |compiler: &mut Compiler<'ctx>| {
            let string_field = variable.get_field(compiler, Field::String);
            let string_field = compiler
                .builder
                .build_load(string_field, "")
                .into_pointer_value();
            compiler.builder.build_call(
                self.func,
                &[
                    compiler
                        .builder
                        .build_pointer_cast(
                            self.p_str_fmt.as_pointer_value(),
                            compiler.context.i8_type().ptr_type(AddressSpace::Generic),
                            "",
                        )
                        .into(),
                    string_field.into(),
                ],
                "",
            );
        };

        variable.switch_type(compiler, cur_function, number_case_f, string_case_f);
        Ok(())
    }
}
