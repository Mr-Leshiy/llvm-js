use crate::{ptr::Ptr, variable::Variable};
use std::process::abort;

#[no_mangle]
pub unsafe extern "C" fn variable_assert(this: *mut Variable) {
    let this = Ptr::from_raw(this).unwrap();

    if !this.get_ref().to_boolean() {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn variable_assert_eq(val1: *mut Variable, val2: *mut Variable) {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    if val1.get_ref() != val2.get_ref() {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn print(this: *mut Variable) {
    let this = Ptr::from_raw(this).unwrap();

    println!("{}", this.get_ref().to_string());
}
