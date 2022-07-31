use super::Compiler;
use crate::Function;
use inkwell::{
    types::StructType,
    values::{IntValue, PointerValue},
    AddressSpace,
};

#[derive(Clone, Copy)]
pub(crate) enum Type {
    Number = 0,
    String = 1,
    Boolean = 2,
}

impl Type {
    fn to_int<'ctx, T>(self, compiler: &Compiler<'ctx, T>) -> IntValue<'ctx> {
        compiler.context.i8_type().const_int(self as u64, false)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Field {
    Flag = 0,
    Number = 1,
    String = 2,
    Boolean = 3,
}

#[derive(Clone)]
pub struct Variable<'ctx> {
    pub(crate) value: PointerValue<'ctx>,
}

impl<'ctx> Variable<'ctx> {
    fn new<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = Self::get_type(compiler);

        let value = compiler.builder.build_alloca(var_type, "");
        Self { value }
    }

    pub(crate) fn get_type<T>(compiler: &Compiler<'ctx, T>) -> StructType<'ctx> {
        let number_type = compiler.context.f64_type();
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);
        let boolean_type = compiler.context.bool_type();
        let type_flag_type = compiler.context.i8_type();

        compiler.context.struct_type(
            &[
                type_flag_type.into(),
                number_type.into(),
                string_type.into(),
                boolean_type.into(),
            ],
            false,
        )
    }

    pub(crate) fn get_field<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        field: Field,
    ) -> PointerValue<'ctx> {
        compiler
            .builder
            .build_struct_gep(self.value, field as u32, "")
            .expect(
                "enum Field values must converge to the struct_type() type initialization order",
            )
    }

    fn update_flag<T>(&self, compiler: &Compiler<'ctx, T>, t: Type) {
        let flag_field = self.get_field(compiler, Field::Flag);
        let t = t.to_int(compiler);
        compiler.builder.build_store(flag_field, t);
    }

    fn get_flag<T>(&self, compiler: &Compiler<'ctx, T>) -> IntValue<'ctx> {
        let flag_field = self.get_field(compiler, Field::Flag);
        compiler.builder.build_load(flag_field, "").into_int_value()
    }

    pub(crate) fn switch_type<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        cur_function: &Function<'ctx, T>,
        number_case_f: impl FnOnce(&Compiler<'ctx, T>),
        string_case_f: impl FnOnce(&Compiler<'ctx, T>),
        boolean_case_f: impl FnOnce(&Compiler<'ctx, T>),
    ) {
        let flag = self.get_flag(compiler);

        let else_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let number_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let string_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let boolean_block = compiler
            .context
            .append_basic_block(cur_function.function, "");

        let continue_block = compiler
            .context
            .append_basic_block(cur_function.function, "");

        let number_case = (Type::Number.to_int(compiler), number_block);
        let string_case = (Type::String.to_int(compiler), string_block);
        let boolean_case = (Type::Boolean.to_int(compiler), boolean_block);

        compiler
            .builder
            .build_switch(flag, else_block, &[number_case, string_case, boolean_case]);

        // describe else case
        compiler.builder.position_at_end(else_block);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe number case
        compiler.builder.position_at_end(number_block);
        number_case_f(compiler);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe string case
        compiler.builder.position_at_end(string_block);
        string_case_f(compiler);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe boolean case
        compiler.builder.position_at_end(boolean_block);
        boolean_case_f(compiler);
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn new_number<T>(compiler: &Compiler<'ctx, T>, number: f64) -> Self {
        let variable = Self::new(compiler);
        variable.assign_number(compiler, number);
        variable
    }

    pub fn assign_number<T>(&self, compiler: &Compiler<'ctx, T>, number: f64) {
        let number = compiler.context.f64_type().const_float(number);
        let number_field = self.get_field(compiler, Field::Number);
        compiler.builder.build_store(number_field, number);
        self.update_flag(compiler, Type::Number);
    }

    pub fn new_string<T>(compiler: &Compiler<'ctx, T>, string: &str) -> Self {
        let variable = Self::new(compiler);
        variable.assign_string(compiler, string);
        variable
    }

    pub fn assign_string<T>(&self, compiler: &Compiler<'ctx, T>, string: &str) {
        let string = compiler.context.const_string(string.as_bytes(), true);
        let tmp_value = compiler.builder.build_alloca(string.get_type(), "");
        compiler.builder.build_store(tmp_value, string);
        let tmp_value = compiler
            .builder
            .build_bitcast(
                tmp_value,
                compiler.context.i8_type().ptr_type(AddressSpace::Generic),
                "",
            )
            .into_pointer_value();

        let string_field = self.get_field(compiler, Field::String);
        compiler.builder.build_store(string_field, tmp_value);
        self.update_flag(compiler, Type::String);
    }

    pub fn new_boolean<T>(compiler: &Compiler<'ctx, T>, boolean: bool) -> Self {
        let variable = Self::new(compiler);
        variable.assign_boolean(compiler, boolean);
        variable
    }

    pub fn assign_boolean<T>(&self, compiler: &Compiler<'ctx, T>, boolean: bool) {
        let boolean = compiler
            .context
            .bool_type()
            .const_int(boolean.then_some(1_u64).unwrap_or(0_u64), false);
        let boolean_field = self.get_field(compiler, Field::Boolean);
        compiler.builder.build_store(boolean_field, boolean);
        self.update_flag(compiler, Type::Boolean);
    }

    pub fn new_variable<T>(
        compiler: &mut Compiler<'ctx, T>,
        cur_function: &Function<'ctx, T>,
        variable2: &Self,
    ) -> Self {
        let variable1 = Self::new(compiler);
        variable1.assign_variable(compiler, cur_function, variable2);
        variable1
    }

    pub fn assign_variable<T>(
        &self,
        compiler: &mut Compiler<'ctx, T>,
        cur_function: &Function<'ctx, T>,
        variable: &Self,
    ) {
        let number_case_f = |compiler: &Compiler<'ctx, T>| {
            self.update_flag(compiler, Type::Number);
            let self_filed = self.get_field(compiler, Field::Number);
            let variable_field = variable.get_field(compiler, Field::Number);
            let variable_field = compiler
                .builder
                .build_load(variable_field, "")
                .into_float_value();
            compiler.builder.build_store(self_filed, variable_field);
        };
        let string_case_f = |compiler: &Compiler<'ctx, T>| {
            self.update_flag(compiler, Type::String);
            let self_filed = self.get_field(compiler, Field::String);
            let variable_field = variable.get_field(compiler, Field::String);
            let variable_field = compiler
                .builder
                .build_load(variable_field, "")
                .into_pointer_value();
            compiler.builder.build_store(self_filed, variable_field);
        };
        let boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            self.update_flag(compiler, Type::Boolean);
            let self_filed = self.get_field(compiler, Field::Boolean);
            let variable_field = variable.get_field(compiler, Field::Boolean);
            let variable_field = compiler
                .builder
                .build_load(variable_field, "")
                .into_int_value();
            compiler.builder.build_store(self_filed, variable_field);
        };

        variable.switch_type(
            compiler,
            cur_function,
            number_case_f,
            string_case_f,
            boolean_case_f,
        );
    }
}
