use super::Compiler;
use inkwell::{
    types::StructType,
    values::{FunctionValue, IntValue, PointerValue},
    AddressSpace,
};

pub enum Type {
    Number = 0,
    String = 1,
}

impl Type {
    fn to_int<'ctx>(self, compiler: &mut Compiler<'ctx>) -> IntValue<'ctx> {
        compiler.context.i8_type().const_int(self as u64, false)
    }
}

pub enum Field {
    Flag = 0,
    Number = 1,
    String = 2,
}

#[derive(Clone)]
pub struct Variable<'ctx> {
    pub(super) value: PointerValue<'ctx>,
}

impl<'ctx> Variable<'ctx> {
    fn get_type(compiler: &mut Compiler<'ctx>) -> StructType<'ctx> {
        let number_type = compiler.context.f64_type();
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);
        let type_flag_type = compiler.context.i8_type();

        compiler.context.struct_type(
            &[
                type_flag_type.into(),
                number_type.into(),
                string_type.into(),
            ],
            false,
        )
    }

    fn new(compiler: &mut Compiler<'ctx>, name: &str) -> Self {
        let var_type = Self::get_type(compiler);

        let value = compiler.builder.build_alloca(var_type, name);
        Self { value }
    }

    fn get_field(&self, compiler: &mut Compiler<'ctx>, field: Field) -> PointerValue<'ctx> {
        compiler
            .builder
            .build_struct_gep(self.value, field as u32, "")
            .expect(
                "enum Field values must converge to the struct_type() type initialization order",
            )
    }

    fn update_flag(&self, compiler: &mut Compiler<'ctx>, t: Type) {
        let flag_field = self.get_field(compiler, Field::Flag);
        let t = t.to_int(compiler);
        compiler.builder.build_store(flag_field, t);
    }

    fn get_flag(&self, compiler: &mut Compiler<'ctx>) -> IntValue<'ctx> {
        let flag_field = self.get_field(compiler, Field::Flag);
        compiler.builder.build_load(flag_field, "").into_int_value()
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn new_number(compiler: &mut Compiler<'ctx>, number: f64, name: &str) -> Self {
        let variable = Self::new(compiler, name);
        variable.assign_number(compiler, number);
        variable
    }

    pub fn assign_number(&self, compiler: &mut Compiler<'ctx>, number: f64) {
        let number = compiler.context.f64_type().const_float(number);
        let number_field = self.get_field(compiler, Field::Number);
        compiler.builder.build_store(number_field, number);
        self.update_flag(compiler, Type::Number);
    }

    pub fn new_string(compiler: &mut Compiler<'ctx>, string: &str, name: &str) -> Self {
        let variable = Self::new(compiler, name);
        variable.assign_string(compiler, string);
        variable
    }

    pub fn assign_string(&self, compiler: &mut Compiler<'ctx>, string: &str) {
        let string = compiler.context.const_string(string.as_bytes(), false);
        let string = string
            .get_element_as_constant(0)
            .into_int_value()
            .const_to_pointer(compiler.context.i8_type().ptr_type(AddressSpace::Generic));

        let string_field = self.get_field(compiler, Field::String);
        compiler.builder.build_store(string_field, string);
        self.update_flag(compiler, Type::String);
    }

    pub fn new_variable(compiler: &mut Compiler<'ctx>, name: &str, _variable2: &Variable) -> Self {
        let variable1 = Self::new(compiler, name);
        // variable1.assign_variable(compiler, variable2);
        variable1
    }

    pub fn assign_variable(
        &self,
        compiler: &mut Compiler<'ctx>,
        variable: &Variable<'ctx>,
        cur_func: FunctionValue<'ctx>,
    ) {
        let flag = variable.get_flag(compiler);

        let else_block = compiler
            .context
            .append_basic_block(cur_func, "assing_variable_else_block");
        let number_block = compiler
            .context
            .append_basic_block(cur_func, "assing_variable_number_block");
        let string_block = compiler
            .context
            .append_basic_block(cur_func, "assing_variable_string_block");

        let number_case = (Type::Number.to_int(compiler), number_block);
        let string_case = (Type::String.to_int(compiler), string_block);

        compiler
            .builder
            .build_switch(flag, else_block, &[number_case, string_case]);

        // describe number case
        compiler.builder.position_at_end(number_block);

        self.update_flag(compiler, Type::Number);
        let self_filed = self.get_field(compiler, Field::Number);
        let variable_field = variable.get_field(compiler, Field::Number);
        let variable_field = compiler
            .builder
            .build_load(variable_field, "")
            .into_int_value();
        compiler.builder.build_store(self_filed, variable_field);

        // describe string case
        compiler.builder.position_at_end(string_block);

        self.update_flag(compiler, Type::String);
        let self_filed = self.get_field(compiler, Field::String);
        let variable_field = variable.get_field(compiler, Field::String);
        compiler.builder.build_store(self_filed, variable_field);
    }
}
