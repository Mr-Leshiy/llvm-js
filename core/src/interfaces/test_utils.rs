use crate::{ptr::RawPtr, variable::Variable};
use std::process::abort;

#[no_mangle]
pub unsafe extern "C" fn variable_assert(this: *mut Variable) {
    let this = RawPtr::from_raw(this).unwrap();

    if !this.to_boolean() {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn variable_assert_eq(val1: *mut Variable, val2: *mut Variable) {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    if val1 != val2 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn print(this: *mut Variable) {
    let this = RawPtr::from_raw(this).unwrap();

    println!("{}", this.to_string());
}
