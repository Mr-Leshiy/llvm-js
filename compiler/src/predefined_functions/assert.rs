use super::{abort::AbortFn, Compiler, PredefineFunctionName};
use crate::{variable::Field, Error, Function, Variable, VariableValue};

#[derive(Clone)]
pub struct AssertFn;

impl PredefineFunctionName for AssertFn {
    const NAME: &'static str = "assert";
}

impl AssertFn {
    pub(super) fn declare() -> Self {
        Self
    }
}

impl AssertFn {
    pub fn assert<'ctx>(
        &self,
        compiler: &Compiler<'ctx>,
        cur_function: &Function<'ctx>,
        abort_fn: &AbortFn<'ctx>,
        arg: VariableValue,
    ) -> Result<(), Error> {
        let variable = Variable::try_from_variable_value(compiler, cur_function, arg)?;

        let number_case_f = |_compiler: &Compiler<'ctx>| {
            // TODO implement
            abort_fn.abort(compiler);
        };
        let string_case_f = |_compiler: &Compiler<'ctx>| {
            // TODO implement
            abort_fn.abort(compiler);
        };
        let boolean_case_f = |compiler: &Compiler<'ctx>| {
            let boolean_field = variable.get_field(compiler, Field::Boolean);
            let boolean_field = compiler
                .builder
                .build_load(boolean_field, "")
                .into_int_value();

            let true_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let false_block = compiler
                .context
                .append_basic_block(cur_function.function, "");

            compiler
                .builder
                .build_conditional_branch(boolean_field, true_block, false_block);

            // describe false case
            compiler.builder.position_at_end(false_block);
            abort_fn.abort(compiler);
            compiler.builder.build_unconditional_branch(true_block);

            // describe true case
            compiler.builder.position_at_end(true_block);
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
