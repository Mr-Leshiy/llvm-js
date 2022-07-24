use super::{Compiler, ExternFunction, ExternFunctionName};
use crate::{variable::Field, Error, Function, Variable, VariableValue};
use inkwell::{
    module::Linkage,
    values::{FunctionValue, GlobalValue},
    AddressSpace,
};

#[derive(Clone)]
pub struct PrintFn<'ctx> {
    func: FunctionValue<'ctx>,

    p_f64_fmt: GlobalValue<'ctx>,
    p_boolean_fmt: GlobalValue<'ctx>,
    p_str_fmt: GlobalValue<'ctx>,
}

impl<'ctx> ExternFunction<'ctx> for PrintFn<'ctx> {}

impl<'ctx> ExternFunctionName<'ctx> for PrintFn<'ctx> {
    const NAME: &'static str = "print";
}

impl<'ctx> PrintFn<'ctx> {
    pub(super) fn declare(compiler: &Compiler<'ctx>) -> Self {
        let s = compiler.context.const_string(b"%f\n", true);
        let p_f64_fmt = compiler.module.add_global(s.get_type(), None, "p_f64_fmt");
        p_f64_fmt.set_constant(true);
        p_f64_fmt.set_externally_initialized(false);
        p_f64_fmt.set_initializer(&s);

        let s = compiler.context.const_string(b"%d\n", true);
        let p_boolean_fmt = compiler
            .module
            .add_global(s.get_type(), None, "p_boolean_fmt");
        p_boolean_fmt.set_constant(true);
        p_boolean_fmt.set_externally_initialized(false);
        p_boolean_fmt.set_initializer(&s);

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
            p_boolean_fmt,
            p_str_fmt,
        }
    }

    pub fn print(
        &self,
        compiler: &Compiler<'ctx>,
        cur_function: &Function<'ctx>,
        arg: VariableValue,
    ) -> Result<(), Error> {
        let variable = Variable::try_from_variable_value(compiler, cur_function, arg)?;

        let number_case_f = |compiler: &Compiler<'ctx>| {
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
        let string_case_f = |compiler: &Compiler<'ctx>| {
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
        let boolean_case_f = |compiler: &Compiler<'ctx>| {
            let boolean_field = variable.get_field(compiler, Field::Boolean);
            let boolean_field = compiler
                .builder
                .build_load(boolean_field, "")
                .into_int_value();
            compiler.builder.build_call(
                self.func,
                &[
                    compiler
                        .builder
                        .build_pointer_cast(
                            self.p_boolean_fmt.as_pointer_value(),
                            compiler.context.i8_type().ptr_type(AddressSpace::Generic),
                            "",
                        )
                        .into(),
                    boolean_field.into(),
                ],
                "",
            );
        };

        variable.switch_type(
            compiler,
            cur_function,
            number_case_f,
            string_case_f,
            boolean_case_f,
        );
        Ok(())
    }
}
