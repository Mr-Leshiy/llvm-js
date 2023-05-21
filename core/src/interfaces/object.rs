use crate::{ptr::RawPtr, variable::{VariableValue, Variable}};
use std::{
    ffi::{c_char, CStr},
    ops::Deref,
};

#[no_mangle]
pub unsafe extern "C" fn add_property_by_boolean(
    this: *mut Variable,
    property_name: bool,
    property: *mut Variable,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(&property_name.into(), property);
}

#[no_mangle]
pub unsafe extern "C" fn add_property_by_number(
    this: *mut Variable,
    property_name: f64,
    property: *mut Variable,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(&VariableValue::Number(property_name.into()), property);
}

#[no_mangle]
pub unsafe extern "C" fn add_property_by_str(
    this: *mut Variable,
    property_name: *const c_char,
    property: *mut Variable,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = CStr::from_ptr(property_name).to_str().unwrap().to_string();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(&property_name.into(), property);
}

#[no_mangle]
pub extern "C" fn add_property_by_var(
    this: *mut Variable,
    property_name: *mut Variable,
    property: *mut Variable,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = RawPtr::from_raw(property_name).unwrap();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(property_name.deref(), property);
}

#[no_mangle]
pub unsafe extern "C" fn get_property_by_boolean(
    this: *mut Variable,
    property_name: bool,
) -> *mut Variable {
    let mut this = RawPtr::from_raw(this).unwrap();

    this.get_property(&property_name.into()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn get_property_by_number(
    this: *mut Variable,
    property_name: f64,
) -> *mut Variable {
    let mut this = RawPtr::from_raw(this).unwrap();

    this.get_property(&VariableValue::Number(property_name.into()))
        .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn get_property_by_str(
    this: *mut Variable,
    property_name: *const c_char,
) -> *mut Variable {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = CStr::from_ptr(property_name).to_str().unwrap().to_string();

    this.get_property(&property_name.into()).get_raw()
}

#[no_mangle]
pub extern "C" fn get_property_by_var(
    this: *mut Variable,
    property_name: *mut Variable,
) -> *mut Variable {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = RawPtr::from_raw(property_name).unwrap();

    this.get_property(property_name.deref()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn function_call(
    this: *mut Variable,
    args: *mut *mut Variable,
    args_num: u32,
) -> *mut Variable {
    let this = RawPtr::from_raw(this).unwrap();
    // providing 0 capacity value because we dont want to deallocate memory for args
    let mut args = Vec::from_raw_parts(args, args_num as usize, 0);

    this.function_call(&mut args).get_raw()
}
