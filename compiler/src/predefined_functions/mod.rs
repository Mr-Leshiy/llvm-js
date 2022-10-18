use self::{
    arithmetic::{
        ArithmeticAdditionFn, ArithmeticDivisionFn, ArithmeticMultiplicationFn,
        ArithmeticSubstractionFn,
    },
    assertions::{AssertEqFn, AssertFn},
    convert::{ConvertToBooleanFn, ConvertToNumberFn, ConvertToStringFn},
    logical::{
        LogicalAndFn, LogicalEqFn, LogicalNeFn, LogicalNotFn, LogicalOrFn, LogicalSEqFn,
        LogicalSNeFn,
    },
    variable::{
        AddPropertyFn, AllocateFn, GetBooleanFn, GetPropertyByStrFn, GetPropertyByVarFn,
        InitObjectFn, PrintFn, RemovePropertyFn, SetBooleanFn, SetInfinityFn, SetNaNFn,
        SetNegInfinityFn, SetNullFn, SetNumberFn, SetStringFn, SetUndefinedFn, SetVariableFn,
    },
};
use crate::Compiler;

pub mod arithmetic;
pub mod assertions;
pub mod convert;
pub mod logical;
pub mod variable;

pub trait PredefineFunctionName {
    const NAME: &'static str;
}

pub struct PredefineFunctions<'ctx> {
    // assertion functions
    assert: AssertFn<'ctx>,
    assert_eq: AssertEqFn<'ctx>,
    // variable functions
    allocate: AllocateFn<'ctx>,
    set_undefined: SetUndefinedFn<'ctx>,
    set_null: SetNullFn<'ctx>,
    set_nan: SetNaNFn<'ctx>,
    set_infinity: SetInfinityFn<'ctx>,
    set_neginfinity: SetNegInfinityFn<'ctx>,
    set_number: SetNumberFn<'ctx>,
    set_boolean: SetBooleanFn<'ctx>,
    set_string: SetStringFn<'ctx>,
    set_variable: SetVariableFn<'ctx>,
    get_boolean: GetBooleanFn<'ctx>,
    printf: PrintFn<'ctx>,
    // object functions
    init_object: InitObjectFn<'ctx>,
    add_property: AddPropertyFn<'ctx>,
    get_property_by_str: GetPropertyByStrFn<'ctx>,
    get_property_by_var: GetPropertyByVarFn<'ctx>,
    remove_property: RemovePropertyFn<'ctx>,
    // logical functions
    logical_not: LogicalNotFn<'ctx>,
    logical_and: LogicalAndFn<'ctx>,
    logical_or: LogicalOrFn<'ctx>,
    logical_eq: LogicalEqFn<'ctx>,
    logical_ne: LogicalNeFn<'ctx>,
    logical_seq: LogicalSEqFn<'ctx>,
    logical_sne: LogicalSNeFn<'ctx>,
    // arithmetic functions
    arithmetic_addition: ArithmeticAdditionFn<'ctx>,
    arithmetic_substraction: ArithmeticSubstractionFn<'ctx>,
    arithmetic_multiplication: ArithmeticMultiplicationFn<'ctx>,
    arithmetic_division: ArithmeticDivisionFn<'ctx>,
    // convert
    convert_to_boolean: ConvertToBooleanFn<'ctx>,
    convert_to_number: ConvertToNumberFn<'ctx>,
    convert_to_string: ConvertToStringFn<'ctx>,
}

impl<'ctx> PredefineFunctions<'ctx> {
    pub(crate) fn declare<T>(compiler: &mut Compiler<'ctx, T>) -> Self {
        Self {
            // assertion functions
            assert: AssertFn::declare(compiler),
            assert_eq: AssertEqFn::declare(compiler),
            // variable functions
            allocate: AllocateFn::declare(compiler),
            set_undefined: SetUndefinedFn::declare(compiler),
            set_null: SetNullFn::declare(compiler),
            set_nan: SetNaNFn::declare(compiler),
            set_infinity: SetInfinityFn::declare(compiler),
            set_neginfinity: SetNegInfinityFn::declare(compiler),
            set_number: SetNumberFn::declare(compiler),
            set_boolean: SetBooleanFn::declare(compiler),
            set_string: SetStringFn::declare(compiler),
            set_variable: SetVariableFn::declare(compiler),
            get_boolean: GetBooleanFn::declare(compiler),
            printf: PrintFn::declare(compiler),
            // object functions
            init_object: InitObjectFn::declare(compiler),
            add_property: AddPropertyFn::declare(compiler),
            get_property_by_str: GetPropertyByStrFn::declare(compiler),
            get_property_by_var: GetPropertyByVarFn::declare(compiler),
            remove_property: RemovePropertyFn::declare(compiler),
            // logical functions
            logical_not: LogicalNotFn::declare(compiler),
            logical_and: LogicalAndFn::declare(compiler),
            logical_or: LogicalOrFn::declare(compiler),
            logical_eq: LogicalEqFn::declare(compiler),
            logical_ne: LogicalNeFn::declare(compiler),
            logical_seq: LogicalSEqFn::declare(compiler),
            logical_sne: LogicalSNeFn::declare(compiler),
            // arithmetic functions
            arithmetic_addition: ArithmeticAdditionFn::declare(compiler),
            arithmetic_substraction: ArithmeticSubstractionFn::declare(compiler),
            arithmetic_multiplication: ArithmeticMultiplicationFn::declare(compiler),
            arithmetic_division: ArithmeticDivisionFn::declare(compiler),
            // convert
            convert_to_boolean: ConvertToBooleanFn::declare(compiler),
            convert_to_number: ConvertToNumberFn::declare(compiler),
            convert_to_string: ConvertToStringFn::declare(compiler),
        }
    }

    // assetion functions
    pub fn assert(&self) -> &AssertFn<'ctx> {
        &self.assert
    }

    pub fn assert_eq(&self) -> &AssertEqFn<'ctx> {
        &self.assert_eq
    }

    // variable functions
    pub fn allocate(&self) -> &AllocateFn<'ctx> {
        &self.allocate
    }

    pub fn set_undefined(&self) -> &SetUndefinedFn<'ctx> {
        &self.set_undefined
    }

    pub fn set_null(&self) -> &SetNullFn<'ctx> {
        &self.set_null
    }

    pub fn set_nan(&self) -> &SetNaNFn<'ctx> {
        &self.set_nan
    }

    pub fn set_infinity(&self) -> &SetInfinityFn<'ctx> {
        &self.set_infinity
    }

    pub fn set_neginfinity(&self) -> &SetNegInfinityFn<'ctx> {
        &self.set_neginfinity
    }

    pub fn set_number(&self) -> &SetNumberFn<'ctx> {
        &self.set_number
    }

    pub fn set_boolean(&self) -> &SetBooleanFn<'ctx> {
        &self.set_boolean
    }

    pub fn set_string(&self) -> &SetStringFn<'ctx> {
        &self.set_string
    }

    pub fn set_variable(&self) -> &SetVariableFn<'ctx> {
        &self.set_variable
    }

    pub fn get_boolean(&self) -> &GetBooleanFn<'ctx> {
        &self.get_boolean
    }

    pub fn print(&self) -> &PrintFn<'ctx> {
        &self.printf
    }

    // object functions
    pub fn init_object(&self) -> &InitObjectFn<'ctx> {
        &self.init_object
    }

    pub fn add_property(&self) -> &AddPropertyFn<'ctx> {
        &self.add_property
    }

    pub fn get_property_by_str(&self) -> &GetPropertyByStrFn<'ctx> {
        &self.get_property_by_str
    }

    pub fn get_property_by_var(&self) -> &GetPropertyByVarFn<'ctx> {
        &self.get_property_by_var
    }

    pub fn remove_property(&self) -> &RemovePropertyFn<'ctx> {
        &self.remove_property
    }

    // logical functions
    pub fn logical_not(&self) -> &LogicalNotFn<'ctx> {
        &self.logical_not
    }

    pub fn logical_and(&self) -> &LogicalAndFn<'ctx> {
        &self.logical_and
    }

    pub fn logical_or(&self) -> &LogicalOrFn<'ctx> {
        &self.logical_or
    }

    pub fn logical_eq(&self) -> &LogicalEqFn<'ctx> {
        &self.logical_eq
    }

    pub fn logical_ne(&self) -> &LogicalNeFn<'ctx> {
        &self.logical_ne
    }

    pub fn logical_seq(&self) -> &LogicalSEqFn<'ctx> {
        &self.logical_seq
    }

    pub fn logical_sne(&self) -> &LogicalSNeFn<'ctx> {
        &self.logical_sne
    }

    // arithmetic functions
    pub fn arithmetic_addition(&self) -> &ArithmeticAdditionFn<'ctx> {
        &self.arithmetic_addition
    }

    pub fn arithmetic_substraction(&self) -> &ArithmeticSubstractionFn<'ctx> {
        &self.arithmetic_substraction
    }

    pub fn arithmetic_multiplication(&self) -> &ArithmeticMultiplicationFn<'ctx> {
        &self.arithmetic_multiplication
    }

    pub fn arithmetic_division(&self) -> &ArithmeticDivisionFn<'ctx> {
        &self.arithmetic_division
    }

    // convert
    pub fn convert_to_boolean(&self) -> &ConvertToBooleanFn<'ctx> {
        &self.convert_to_boolean
    }

    pub fn convert_to_number(&self) -> &ConvertToNumberFn<'ctx> {
        &self.convert_to_number
    }

    pub fn convert_to_string(&self) -> &ConvertToStringFn<'ctx> {
        &self.convert_to_string
    }
}
