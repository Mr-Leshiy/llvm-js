use super::{Compiler, PredefineFunction, PredefineFunctionName};
use crate::{variable::Field, Error, Function, Variable, VariableValue};
use inkwell::values::FunctionValue;

#[derive(Clone)]
pub struct AssertFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> PredefineFunctionName<'ctx> for AssertFn<'ctx> {
    const NAME: &'static str = "assert";
}

impl<'ctx> PredefineFunction<'ctx> for AssertFn<'ctx> {
    fn declare(compiler: &Compiler<'ctx>) -> Self {
        let function_type = compiler
            .context
            .void_type()
            .fn_type(&[compiler.context.bool_type().into()], false);
        let func = compiler.module.add_function("assert", function_type, None);

        let res = Self { func };
        res.generate_body(compiler);
        res
    }
}

impl<'ctx> AssertFn<'ctx> {
    fn generate_body(&self, compiler: &Compiler<'ctx>) {
        let basic_block = compiler.context.append_basic_block(self.func, "entry");
        compiler.builder.position_at_end(basic_block);

        let true_block = compiler.context.append_basic_block(self.func, "");
        let false_block = compiler.context.append_basic_block(self.func, "");

        let arg_value = self.func.get_params().get(0).expect("").into_int_value();

        compiler
            .builder
            .build_conditional_branch(arg_value, true_block, false_block);

        // describe true case
        compiler.builder.position_at_end(true_block);
        compiler.builder.build_return(None);

        // describe false case
        compiler.builder.position_at_end(false_block);
        compiler.builder.build_return(None);
    }

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
