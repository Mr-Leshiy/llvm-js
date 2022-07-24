use super::{Compiler, ExternFunction, ExternFunctionName};
use crate::{variable::Field, Error, Function, Variable, VariableValue};
use inkwell::{module::Linkage, values::FunctionValue};

#[derive(Clone)]
pub struct AssertFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> ExternFunction<'ctx> for AssertFn<'ctx> {
    fn declare(compiler: &Compiler<'ctx>) -> Self {
        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[compiler.context.i32_type().into()], false);
        let func = compiler
            .module
            .add_function("assert", function_type, Some(Linkage::External));

        Self { func }
    }
}

impl<'ctx> ExternFunctionName<'ctx> for AssertFn<'ctx> {
    const NAME: &'static str = "assert";
}

impl<'ctx> AssertFn<'ctx> {
    pub fn assert(
        &self,
        compiler: &Compiler<'ctx>,
        cur_function: &Function<'ctx>,
        arg: VariableValue,
    ) -> Result<(), Error> {
        let variable = Variable::try_from_variable_value(compiler, cur_function, arg)?;

        let number_case_f = |_compiler: &Compiler<'ctx>| {
            // TODO implement
        };
        let string_case_f = |_compiler: &Compiler<'ctx>| {
            // TODO implement
        };
        let boolean_case_f = |compiler: &Compiler<'ctx>| {
            let boolean_field = variable.get_field(compiler, Field::Boolean);
            let boolean_field = compiler
                .builder
                .build_load(boolean_field, "")
                .into_int_value();
            compiler
                .builder
                .build_call(self.func, &[boolean_field.into()], "");
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
