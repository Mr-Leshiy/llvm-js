use super::Compiler;
use crate::{
    types::{Boolean, Float, IntegerI8, String, ValueType},
    Function,
};
use inkwell::{
    types::StructType,
    values::{IntValue, PointerValue},
};

#[derive(Clone)]
pub struct Variable<'ctx> {
    pub(crate) value: PointerValue<'ctx>,
}

pub(crate) trait FieldTrait<'ctx> {
    type Type: ValueType<'ctx>;
    const INDEX: u32;

    fn index_to_int<T>(compiler: &Compiler<'ctx, T>) -> IntValue<'ctx> {
        compiler
            .context
            .i8_type()
            .const_int(Self::INDEX as u64, false)
    }
}

pub(crate) struct FlagField;
impl<'ctx> FieldTrait<'ctx> for FlagField {
    type Type = IntegerI8<'ctx>;
    const INDEX: u32 = 0;
}

pub(crate) struct NumberField;
impl<'ctx> FieldTrait<'ctx> for NumberField {
    type Type = Float<'ctx>;
    const INDEX: u32 = 1;
}

pub(crate) struct StringField;
impl<'ctx> FieldTrait<'ctx> for StringField {
    type Type = String<'ctx>;
    const INDEX: u32 = 2;
}

pub(crate) struct BooleanField;
impl<'ctx> FieldTrait<'ctx> for BooleanField {
    type Type = Boolean<'ctx>;
    const INDEX: u32 = 3;
}

impl<'ctx> Variable<'ctx> {
    pub(crate) fn new<T>(compiler: &Compiler<'ctx, T>) -> Self {
        let var_type = Self::get_type(compiler);

        let value = compiler.builder.build_alloca(var_type, "");
        Self { value }
    }

    pub(crate) fn get_type<T>(compiler: &Compiler<'ctx, T>) -> StructType<'ctx> {
        let number_type = Float::get_type(compiler);
        let string_type = String::get_type(compiler);
        let boolean_type = Boolean::get_type(compiler);
        let type_flag_type = IntegerI8::get_type(compiler);

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

    pub(crate) fn get_type_<T>(compiler: &Compiler<'ctx, T>) -> StructType<'ctx> {
        compiler.context.opaque_struct_type("VariableType")
    }

    pub(crate) fn get_field<T, Field: FieldTrait<'ctx>>(
        &self,
        compiler: &Compiler<'ctx, T>,
    ) -> Field::Type {
        let ptr = compiler
            .builder
            .build_struct_gep(self.value, Field::INDEX as u32, "")
            .expect(
                "enum Field values must converge to the struct_type() type initialization order",
            );
        Field::Type::new(ptr)
    }

    fn update_flag<T, Field: FieldTrait<'ctx>>(&self, compiler: &Compiler<'ctx, T>) {
        let flag_field = self.get_field::<T, FlagField>(compiler);
        flag_field.assign_literal(compiler, Field::INDEX as u64);
    }

    pub(crate) fn switch_type<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        cur_function: &Function<'ctx, T>,
        number_case_f: impl FnOnce(&Compiler<'ctx, T>),
        string_case_f: impl FnOnce(&Compiler<'ctx, T>),
        boolean_case_f: impl FnOnce(&Compiler<'ctx, T>),
    ) {
        let flag_field = self.get_field::<T, FlagField>(compiler);

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

        let number_case = (NumberField::index_to_int(compiler), number_block);
        let string_case = (StringField::index_to_int(compiler), string_block);
        let boolean_case = (BooleanField::index_to_int(compiler), boolean_block);

        compiler.builder.build_switch(
            flag_field.load_value(compiler),
            else_block,
            &[number_case, string_case, boolean_case],
        );

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
        let number_field = self.get_field::<T, NumberField>(compiler);
        number_field.assign_literal(compiler, number);
        self.update_flag::<T, NumberField>(compiler);
    }

    pub fn new_string<T>(compiler: &Compiler<'ctx, T>, string: &str) -> Self {
        let variable = Self::new(compiler);
        variable.assign_string(compiler, string);
        variable
    }

    pub fn assign_string<T>(&self, compiler: &Compiler<'ctx, T>, string: &str) {
        let string_field = self.get_field::<T, StringField>(compiler);
        string_field.assign_literal(compiler, string);
        self.update_flag::<T, StringField>(compiler);
    }

    pub fn new_boolean<T>(compiler: &Compiler<'ctx, T>, boolean: bool) -> Self {
        let variable = Self::new(compiler);
        variable.assign_boolean(compiler, boolean);
        variable
    }

    pub fn assign_boolean<T>(&self, compiler: &Compiler<'ctx, T>, boolean: bool) {
        let boolean_field = self.get_field::<T, BooleanField>(compiler);
        boolean_field.assign_literal(compiler, boolean);
        self.update_flag::<T, BooleanField>(compiler);
    }

    pub fn new_variable<T>(
        compiler: &Compiler<'ctx, T>,
        cur_function: &Function<'ctx, T>,
        variable2: &Self,
    ) -> Self {
        let variable1 = Self::new(compiler);
        variable1.assign_variable(compiler, cur_function, variable2);
        variable1
    }

    pub fn assign_variable<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        cur_function: &Function<'ctx, T>,
        variable: &Self,
    ) {
        let number_case_f = |compiler: &Compiler<'ctx, T>| {
            self.update_flag::<T, NumberField>(compiler);
            let self_field = self.get_field::<T, NumberField>(compiler);
            let variable_field = variable.get_field::<T, NumberField>(compiler);
            self_field.assign_variable(compiler, &variable_field);
        };
        let string_case_f = |compiler: &Compiler<'ctx, T>| {
            self.update_flag::<T, StringField>(compiler);
            let self_field = self.get_field::<T, StringField>(compiler);
            let variable_field = variable.get_field::<T, StringField>(compiler);
            self_field.assign_variable(compiler, &variable_field);
        };
        let boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            self.update_flag::<T, BooleanField>(compiler);
            let self_field = self.get_field::<T, BooleanField>(compiler);
            let variable_field = variable.get_field::<T, BooleanField>(compiler);
            self_field.assign_variable(compiler, &variable_field);
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
