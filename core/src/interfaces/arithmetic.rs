use crate::{
    ptr::RawPtr,
    variable::{Variable, VariableValue},
};
use std::ops::Deref;

#[no_mangle]
pub extern "C" fn arithmetic_addition(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    let res = VariableValue::add(val1.deref(), val2.deref());
    RawPtr::from(Variable::allocate(res)).get_raw()
}

#[no_mangle]
pub extern "C" fn arithmetic_substraction(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    let res = VariableValue::sub(val1.deref(), val2.deref());
    RawPtr::from(Variable::allocate(res)).get_raw()
}

#[no_mangle]
pub extern "C" fn arithmetic_multiplication(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    let res = VariableValue::mul(val1.deref(), val2.deref());
    RawPtr::from(Variable::allocate(res)).get_raw()
}

#[no_mangle]
pub extern "C" fn arithmetic_division(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    let res = VariableValue::div(val1.deref(), val2.deref());
    RawPtr::from(Variable::allocate(res)).get_raw()
}
