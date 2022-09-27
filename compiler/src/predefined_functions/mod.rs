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
        AllocateFn, GetBooleanFn, PrintFn, SetBooleanFn, SetInfinityFn, SetNaNFn, SetNegInfinityFn,
        SetNullFn, SetNumberFn, SetStringFn, SetUndefinedFn, SetVariableFn,
    },
};
use crate::{Compiler, Error};

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
    assert: Option<AssertFn<'ctx>>,
    assert_eq: Option<AssertEqFn<'ctx>>,
    // variable functions
    allocate: Option<AllocateFn<'ctx>>,
    set_undefined: Option<SetUndefinedFn<'ctx>>,
    set_null: Option<SetNullFn<'ctx>>,
    set_nan: Option<SetNaNFn<'ctx>>,
    set_infinity: Option<SetInfinityFn<'ctx>>,
    set_neginfinity: Option<SetNegInfinityFn<'ctx>>,
    set_number: Option<SetNumberFn<'ctx>>,
    set_boolean: Option<SetBooleanFn<'ctx>>,
    set_string: Option<SetStringFn<'ctx>>,
    set_variable: Option<SetVariableFn<'ctx>>,
    get_boolean: Option<GetBooleanFn<'ctx>>,
    printf: Option<PrintFn<'ctx>>,
    // logical functions
    logical_not: Option<LogicalNotFn<'ctx>>,
    logical_and: Option<LogicalAndFn<'ctx>>,
    logical_or: Option<LogicalOrFn<'ctx>>,
    logical_eq: Option<LogicalEqFn<'ctx>>,
    logical_ne: Option<LogicalNeFn<'ctx>>,
    logical_seq: Option<LogicalSEqFn<'ctx>>,
    logical_sne: Option<LogicalSNeFn<'ctx>>,
    // arithmetic functions
    arithmetic_addition: Option<ArithmeticAdditionFn<'ctx>>,
    arithmetic_substraction: Option<ArithmeticSubstractionFn<'ctx>>,
    arithmetic_multiplication: Option<ArithmeticMultiplicationFn<'ctx>>,
    arithmetic_division: Option<ArithmeticDivisionFn<'ctx>>,
    // convert
    convert_to_boolean: Option<ConvertToBooleanFn<'ctx>>,
    convert_to_number: Option<ConvertToNumberFn<'ctx>>,
    convert_to_string: Option<ConvertToStringFn<'ctx>>,
}

impl<'ctx> Default for PredefineFunctions<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ctx> PredefineFunctions<'ctx> {
    pub(crate) fn new() -> Self {
        Self {
            // assertion functions
            assert: None,
            assert_eq: None,
            // variable functions
            allocate: None,
            set_undefined: None,
            set_null: None,
            set_nan: None,
            set_infinity: None,
            set_neginfinity: None,
            set_number: None,
            set_boolean: None,
            set_string: None,
            set_variable: None,
            get_boolean: None,
            printf: None,
            // logical functions
            logical_not: None,
            logical_and: None,
            logical_or: None,
            logical_eq: None,
            logical_ne: None,
            logical_seq: None,
            logical_sne: None,
            // arithmetic functions
            arithmetic_addition: None,
            arithmetic_substraction: None,
            arithmetic_multiplication: None,
            arithmetic_division: None,
            // convert
            convert_to_boolean: None,
            convert_to_number: None,
            convert_to_string: None,
        }
    }

    pub(crate) fn declare<T>(compiler: &mut Compiler<'ctx, T>) -> Result<Self, Error<T>> {
        // assertion functions
        let assert = Some(AssertFn::declare(compiler));
        let assert_eq = Some(AssertEqFn::declare(compiler));
        // variable functions
        let allocate = Some(AllocateFn::declare(compiler));
        let set_undefined = Some(SetUndefinedFn::declare(compiler));
        let set_null = Some(SetNullFn::declare(compiler));
        let set_nan = Some(SetNaNFn::declare(compiler));
        let set_infinity = Some(SetInfinityFn::declare(compiler));
        let set_neginfinity = Some(SetNegInfinityFn::declare(compiler));
        let set_number = Some(SetNumberFn::declare(compiler));
        let set_boolean = Some(SetBooleanFn::declare(compiler));
        let set_string = Some(SetStringFn::declare(compiler));
        let set_variable = Some(SetVariableFn::declare(compiler));
        let get_boolean = Some(GetBooleanFn::declare(compiler));
        let printf = Some(PrintFn::declare(compiler));
        // logical functions
        let logical_not = Some(LogicalNotFn::declare(compiler));
        let logical_and = Some(LogicalAndFn::declare(compiler));
        let logical_or = Some(LogicalOrFn::declare(compiler));
        let logical_eq = Some(LogicalEqFn::declare(compiler));
        let logical_ne = Some(LogicalNeFn::declare(compiler));
        let logical_seq = Some(LogicalSEqFn::declare(compiler));
        let logical_sne = Some(LogicalSNeFn::declare(compiler));
        // arithmetic functions
        let arithmetic_addition = Some(ArithmeticAdditionFn::declare(compiler));
        let arithmetic_substraction = Some(ArithmeticSubstractionFn::declare(compiler));
        let arithmetic_multiplication = Some(ArithmeticMultiplicationFn::declare(compiler));
        let arithmetic_division = Some(ArithmeticDivisionFn::declare(compiler));
        // convert
        let convert_to_boolean = Some(ConvertToBooleanFn::declare(compiler));
        let convert_to_number = Some(ConvertToNumberFn::declare(compiler));
        let convert_to_string = Some(ConvertToStringFn::declare(compiler));

        Ok(Self {
            assert,
            assert_eq,
            allocate,
            set_undefined,
            set_null,
            set_nan,
            set_infinity,
            set_neginfinity,
            set_number,
            set_boolean,
            set_string,
            set_variable,
            get_boolean,
            printf,
            logical_not,
            logical_and,
            logical_or,
            logical_eq,
            logical_ne,
            logical_seq,
            logical_sne,
            arithmetic_addition,
            arithmetic_substraction,
            arithmetic_multiplication,
            arithmetic_division,
            convert_to_boolean,
            convert_to_number,
            convert_to_string,
        })
    }

    fn get_fn<T, FnType: PredefineFunctionName>(
        func: Option<&FnType>,
    ) -> Result<&FnType, Error<T>> {
        func.ok_or_else(|| Error::UndeclaredFunction(FnType::NAME.to_string()))
    }

    // assetion functions
    pub fn assert<T>(&self) -> Result<&AssertFn<'ctx>, Error<T>> {
        Self::get_fn(self.assert.as_ref())
    }

    pub fn assert_eq<T>(&self) -> Result<&AssertEqFn<'ctx>, Error<T>> {
        Self::get_fn(self.assert_eq.as_ref())
    }

    // variable functions
    pub fn allocate<T>(&self) -> Result<&AllocateFn<'ctx>, Error<T>> {
        Self::get_fn(self.allocate.as_ref())
    }

    pub fn set_undefined<T>(&self) -> Result<&SetUndefinedFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_undefined.as_ref())
    }

    pub fn set_null<T>(&self) -> Result<&SetNullFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_null.as_ref())
    }

    pub fn set_nan<T>(&self) -> Result<&SetNaNFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_nan.as_ref())
    }

    pub fn set_infinity<T>(&self) -> Result<&SetInfinityFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_infinity.as_ref())
    }

    pub fn set_neginfinity<T>(&self) -> Result<&SetNegInfinityFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_neginfinity.as_ref())
    }

    pub fn set_number<T>(&self) -> Result<&SetNumberFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_number.as_ref())
    }

    pub fn set_boolean<T>(&self) -> Result<&SetBooleanFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_boolean.as_ref())
    }

    pub fn set_string<T>(&self) -> Result<&SetStringFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_string.as_ref())
    }

    pub fn set_variable<T>(&self) -> Result<&SetVariableFn<'ctx>, Error<T>> {
        Self::get_fn(self.set_variable.as_ref())
    }

    pub fn get_boolean<T>(&self) -> Result<&GetBooleanFn<'ctx>, Error<T>> {
        Self::get_fn(self.get_boolean.as_ref())
    }

    pub fn print<T>(&self) -> Result<&PrintFn<'ctx>, Error<T>> {
        Self::get_fn(self.printf.as_ref())
    }

    // logical functions
    pub fn logical_not<T>(&self) -> Result<&LogicalNotFn<'ctx>, Error<T>> {
        Self::get_fn(self.logical_not.as_ref())
    }

    pub fn logical_and<T>(&self) -> Result<&LogicalAndFn<'ctx>, Error<T>> {
        Self::get_fn(self.logical_and.as_ref())
    }

    pub fn logical_or<T>(&self) -> Result<&LogicalOrFn<'ctx>, Error<T>> {
        Self::get_fn(self.logical_or.as_ref())
    }

    pub fn logical_eq<T>(&self) -> Result<&LogicalEqFn<'ctx>, Error<T>> {
        Self::get_fn(self.logical_eq.as_ref())
    }

    pub fn logical_ne<T>(&self) -> Result<&LogicalNeFn<'ctx>, Error<T>> {
        Self::get_fn(self.logical_ne.as_ref())
    }

    pub fn logical_seq<T>(&self) -> Result<&LogicalSEqFn<'ctx>, Error<T>> {
        Self::get_fn(self.logical_seq.as_ref())
    }

    pub fn logical_sne<T>(&self) -> Result<&LogicalSNeFn<'ctx>, Error<T>> {
        Self::get_fn(self.logical_sne.as_ref())
    }

    // arithmetic functions
    pub fn arithmetic_addition<T>(&self) -> Result<&ArithmeticAdditionFn<'ctx>, Error<T>> {
        Self::get_fn(self.arithmetic_addition.as_ref())
    }

    pub fn arithmetic_substraction<T>(&self) -> Result<&ArithmeticSubstractionFn<'ctx>, Error<T>> {
        Self::get_fn(self.arithmetic_substraction.as_ref())
    }

    pub fn arithmetic_multiplication<T>(
        &self,
    ) -> Result<&ArithmeticMultiplicationFn<'ctx>, Error<T>> {
        Self::get_fn(self.arithmetic_multiplication.as_ref())
    }

    pub fn arithmetic_division<T>(&self) -> Result<&ArithmeticDivisionFn<'ctx>, Error<T>> {
        Self::get_fn(self.arithmetic_division.as_ref())
    }

    // convert
    pub fn convert_to_boolean<T>(&self) -> Result<&ConvertToBooleanFn<'ctx>, Error<T>> {
        Self::get_fn(self.convert_to_boolean.as_ref())
    }

    pub fn convert_to_number<T>(&self) -> Result<&ConvertToNumberFn<'ctx>, Error<T>> {
        Self::get_fn(self.convert_to_number.as_ref())
    }

    pub fn convert_to_string<T>(&self) -> Result<&ConvertToStringFn<'ctx>, Error<T>> {
        Self::get_fn(self.convert_to_string.as_ref())
    }
}
