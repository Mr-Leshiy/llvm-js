use self::{
    alloc::{AllocateFn, DeallocateFn},
    arithmetic::{
        ArithmeticAdditionFn, ArithmeticDivisionFn, ArithmeticMultiplicationFn,
        ArithmeticSubstractionFn,
    },
    convert::{ConvertToBooleanFn, ConvertToNumberFn, ConvertToStringFn},
    logical::{
        LogicalAndFn, LogicalEqFn, LogicalGeFn, LogicalGtFn, LogicalLeFn, LogicalLtFn, LogicalNeFn,
        LogicalNotFn, LogicalOrFn,
    },
    object::{
        AddPropertyByStrFn, AddPropertyByVarFn, FunctionCallFn, GetPropertyByStrFn,
        GetPropertyByVarFn, RemovePropertyFn,
    },
    test_utils::{AssertEqFn, AssertFn, PrintFn},
    variable::{
        GetBooleanFn, SetBooleanFn, SetEmptyArrayFn, SetEmptyObjectFn, SetFunctionFn,
        SetInfinityFn, SetNaNFn, SetNegInfinityFn, SetNullFn, SetNumberFn, SetStringFn,
        SetUndefinedFn, SetVariableFn,
    },
};
use crate::{Compiler, InkwellContext};

pub mod alloc;
pub mod arithmetic;
pub mod convert;
pub mod logical;
pub mod object;
pub mod test_utils;
pub mod variable;

pub struct PredefineFunctions<'ctx> {
    // variable functions
    allocate: AllocateFn<'ctx>,
    deallocate: DeallocateFn<'ctx>,
    set_undefined: SetUndefinedFn<'ctx>,
    set_null: SetNullFn<'ctx>,
    set_nan: SetNaNFn<'ctx>,
    set_empty_object: SetEmptyObjectFn<'ctx>,
    set_empty_array: SetEmptyArrayFn<'ctx>,
    set_infinity: SetInfinityFn<'ctx>,
    set_neginfinity: SetNegInfinityFn<'ctx>,
    set_number: SetNumberFn<'ctx>,
    set_boolean: SetBooleanFn<'ctx>,
    set_string: SetStringFn<'ctx>,
    set_function: SetFunctionFn<'ctx>,
    set_variable: SetVariableFn<'ctx>,
    get_boolean: GetBooleanFn<'ctx>,
    function_call: FunctionCallFn<'ctx>,
    // object functions
    add_property_by_str: AddPropertyByStrFn<'ctx>,
    add_property_by_var: AddPropertyByVarFn<'ctx>,
    get_property_by_str: GetPropertyByStrFn<'ctx>,
    get_property_by_var: GetPropertyByVarFn<'ctx>,
    remove_property: RemovePropertyFn<'ctx>,
    // logical functions
    logical_not: LogicalNotFn<'ctx>,
    logical_and: LogicalAndFn<'ctx>,
    logical_or: LogicalOrFn<'ctx>,
    logical_eq: LogicalEqFn<'ctx>,
    logical_ne: LogicalNeFn<'ctx>,
    logical_gt: LogicalGtFn<'ctx>,
    logical_ge: LogicalGeFn<'ctx>,
    logical_lt: LogicalLtFn<'ctx>,
    logical_le: LogicalLeFn<'ctx>,
    // arithmetic functions
    arithmetic_addition: ArithmeticAdditionFn<'ctx>,
    arithmetic_substraction: ArithmeticSubstractionFn<'ctx>,
    arithmetic_multiplication: ArithmeticMultiplicationFn<'ctx>,
    arithmetic_division: ArithmeticDivisionFn<'ctx>,
    // convert
    convert_to_boolean: ConvertToBooleanFn<'ctx>,
    convert_to_number: ConvertToNumberFn<'ctx>,
    convert_to_string: ConvertToStringFn<'ctx>,
    // testing utils
    assert: AssertFn<'ctx>,
    assert_eq: AssertEqFn<'ctx>,
    printf: PrintFn<'ctx>,
}

impl<'ctx> PredefineFunctions<'ctx> {
    pub(crate) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        Self {
            // variable functions
            allocate: AllocateFn::declare(inkwell_context),
            deallocate: DeallocateFn::declare(inkwell_context),
            set_undefined: SetUndefinedFn::declare(inkwell_context),
            set_null: SetNullFn::declare(inkwell_context),
            set_nan: SetNaNFn::declare(inkwell_context),
            set_empty_object: SetEmptyObjectFn::declare(inkwell_context),
            set_empty_array: SetEmptyArrayFn::declare(inkwell_context),
            set_infinity: SetInfinityFn::declare(inkwell_context),
            set_neginfinity: SetNegInfinityFn::declare(inkwell_context),
            set_number: SetNumberFn::declare(inkwell_context),
            set_boolean: SetBooleanFn::declare(inkwell_context),
            set_string: SetStringFn::declare(inkwell_context),
            set_function: SetFunctionFn::declare(inkwell_context),
            set_variable: SetVariableFn::declare(inkwell_context),
            get_boolean: GetBooleanFn::declare(inkwell_context),
            function_call: FunctionCallFn::declare(inkwell_context),
            // object functions
            add_property_by_str: AddPropertyByStrFn::declare(inkwell_context),
            add_property_by_var: AddPropertyByVarFn::declare(inkwell_context),
            get_property_by_str: GetPropertyByStrFn::declare(inkwell_context),
            get_property_by_var: GetPropertyByVarFn::declare(inkwell_context),
            remove_property: RemovePropertyFn::declare(inkwell_context),
            // logical functions
            logical_not: LogicalNotFn::declare(inkwell_context),
            logical_and: LogicalAndFn::declare(inkwell_context),
            logical_or: LogicalOrFn::declare(inkwell_context),
            logical_eq: LogicalEqFn::declare(inkwell_context),
            logical_ne: LogicalNeFn::declare(inkwell_context),
            logical_gt: LogicalGtFn::declare(inkwell_context),
            logical_ge: LogicalGeFn::declare(inkwell_context),
            logical_lt: LogicalLtFn::declare(inkwell_context),
            logical_le: LogicalLeFn::declare(inkwell_context),
            // arithmetic functions
            arithmetic_addition: ArithmeticAdditionFn::declare(inkwell_context),
            arithmetic_substraction: ArithmeticSubstractionFn::declare(inkwell_context),
            arithmetic_multiplication: ArithmeticMultiplicationFn::declare(inkwell_context),
            arithmetic_division: ArithmeticDivisionFn::declare(inkwell_context),
            // convert
            convert_to_boolean: ConvertToBooleanFn::declare(inkwell_context),
            convert_to_number: ConvertToNumberFn::declare(inkwell_context),
            convert_to_string: ConvertToStringFn::declare(inkwell_context),
            // testing utils
            assert: AssertFn::declare(inkwell_context),
            assert_eq: AssertEqFn::declare(inkwell_context),
            printf: PrintFn::declare(inkwell_context),
        }
    }

    // variable functions
    pub fn allocate(&self) -> &AllocateFn<'ctx> {
        &self.allocate
    }

    pub fn deallocate(&self) -> &DeallocateFn<'ctx> {
        &self.deallocate
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

    pub fn set_empty_object(&self) -> &SetEmptyObjectFn<'ctx> {
        &self.set_empty_object
    }

    pub fn set_empty_array(&self) -> &SetEmptyArrayFn<'ctx> {
        &self.set_empty_array
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

    pub fn set_function(&self) -> &SetFunctionFn<'ctx> {
        &self.set_function
    }

    pub fn set_variable(&self) -> &SetVariableFn<'ctx> {
        &self.set_variable
    }

    pub fn get_boolean(&self) -> &GetBooleanFn<'ctx> {
        &self.get_boolean
    }

    pub fn function_call(&self) -> &FunctionCallFn<'ctx> {
        &self.function_call
    }

    // object functions
    pub fn add_property_by_str(&self) -> &AddPropertyByStrFn<'ctx> {
        &self.add_property_by_str
    }

    pub fn add_property_by_var(&self) -> &AddPropertyByVarFn<'ctx> {
        &self.add_property_by_var
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

    pub fn logical_gt(&self) -> &LogicalGtFn<'ctx> {
        &self.logical_gt
    }

    pub fn logical_ge(&self) -> &LogicalGeFn<'ctx> {
        &self.logical_ge
    }

    pub fn logical_lt(&self) -> &LogicalLtFn<'ctx> {
        &self.logical_lt
    }

    pub fn logical_le(&self) -> &LogicalLeFn<'ctx> {
        &self.logical_le
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

    // testing utils
    pub fn assert(&self) -> &AssertFn<'ctx> {
        &self.assert
    }

    pub fn assert_eq(&self) -> &AssertEqFn<'ctx> {
        &self.assert_eq
    }

    pub fn print(&self) -> &PrintFn<'ctx> {
        &self.printf
    }
}
