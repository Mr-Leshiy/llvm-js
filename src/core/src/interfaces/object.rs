use crate::{ptr::RawPtr, variable::VariableValue};
use std::{
    ffi::{c_char, CStr},
    ops::Deref,
};

#[no_mangle]
pub unsafe extern "C" fn add_property_by_boolean(
    this: *mut VariableValue,
    property_name: bool,
    property: *mut VariableValue,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(&property_name.into(), property);
}

#[no_mangle]
pub unsafe extern "C" fn add_property_by_number(
    this: *mut VariableValue,
    property_name: f64,
    property: *mut VariableValue,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(&VariableValue::Number(property_name.into()), property);
}

#[no_mangle]
pub unsafe extern "C" fn add_property_by_str(
    this: *mut VariableValue,
    property_name: *const c_char,
    property: *mut VariableValue,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = CStr::from_ptr(property_name).to_str().unwrap().to_string();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(&property_name.into(), property);
}

#[no_mangle]
pub extern "C" fn add_property_by_var(
    this: *mut VariableValue,
    property_name: *mut VariableValue,
    property: *mut VariableValue,
) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = RawPtr::from_raw(property_name).unwrap();
    let property = RawPtr::from_raw(property).unwrap();

    this.add_property(property_name.deref(), property);
}

#[no_mangle]
pub unsafe extern "C" fn get_property_by_boolean(
    this: *mut VariableValue,
    property_name: bool,
) -> *mut VariableValue {
    let mut this = RawPtr::from_raw(this).unwrap();

    this.get_property(&property_name.into()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn get_property_by_number(
    this: *mut VariableValue,
    property_name: f64,
) -> *mut VariableValue {
    let mut this = RawPtr::from_raw(this).unwrap();

    this.get_property(&VariableValue::Number(property_name.into()))
        .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn get_property_by_str(
    this: *mut VariableValue,
    property_name: *const c_char,
) -> *mut VariableValue {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = CStr::from_ptr(property_name).to_str().unwrap().to_string();

    this.get_property(&property_name.into()).get_raw()
}

#[no_mangle]
pub extern "C" fn get_property_by_var(
    this: *mut VariableValue,
    property_name: *mut VariableValue,
) -> *mut VariableValue {
    let mut this = RawPtr::from_raw(this).unwrap();
    let property_name = RawPtr::from_raw(property_name).unwrap();

    this.get_property(property_name.deref()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn function_call(
    this: *mut VariableValue,
    args: *mut *mut VariableValue,
    args_num: u32,
) -> *mut VariableValue {
    let this = RawPtr::from_raw(this).unwrap();
    // providing 0 capacity value because we dont want to deallocate memory for args
    let mut args = Vec::from_raw_parts(args, args_num as usize, 0);

    this.function_call(&mut args).get_raw()
}
