use crate::{
    ptr::RawPtr,
    variable::{Variable, VariableValue},
};
use std::ops::Deref;

#[no_mangle]
pub unsafe extern "C" fn logical_not(val: *mut Variable) -> *mut Variable {
    let val = RawPtr::from_raw(val).unwrap();

    RawPtr::from(Variable::allocate(val.not())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_and(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::and(
        val1.deref(),
        val2.deref(),
    )))
    .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_or(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::or(
        val1.deref(),
        val2.deref(),
    )))
    .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_eq(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::from(val1 == val2))).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_ne(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::from(val1 != val2))).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_gt(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::gt(
        val1.deref(),
        val2.deref(),
    )))
    .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_ge(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::ge(
        val1.deref(),
        val2.deref(),
    )))
    .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_lt(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::lt(
        val1.deref(),
        val2.deref(),
    )))
    .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_le(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::from(Variable::allocate(VariableValue::le(
        val1.deref(),
        val2.deref(),
    )))
    .get_raw()
}
