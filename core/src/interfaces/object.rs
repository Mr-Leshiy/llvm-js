use crate::{pointer::Ptr, variable::Variable};
use std::{
    ffi::{c_char, CStr},
    slice::from_raw_parts_mut,
};

#[no_mangle]
pub unsafe extern "C" fn add_property_by_str(
    this: *mut Variable,
    property_name: *const c_char,
    property: *mut Variable,
) {
    let mut this = Ptr::from_raw(this).unwrap();
    let property_name = CStr::from_ptr(property_name).to_str().unwrap().to_string();
    let property = Ptr::from_raw(property).unwrap();

    this.get_mut_ref().add_property(property_name, property);
}

#[no_mangle]
pub extern "C" fn add_property_by_var(
    this: *mut Variable,
    property_name: *mut Variable,
    property: *mut Variable,
) {
    let mut this = Ptr::from_raw(this).unwrap();
    let property_name = Ptr::from_raw(property_name).unwrap();
    let property = Ptr::from_raw(property).unwrap();

    this.get_mut_ref()
        .add_property(property_name.get_ref().to_string(), property);
}

#[no_mangle]
pub unsafe extern "C" fn get_property_by_str(
    this: *mut Variable,
    property_name: *const c_char,
) -> *mut Variable {
    let this = Ptr::from_raw(this).unwrap();
    let property_name = CStr::from_ptr(property_name).to_str().unwrap().to_string();

    this.get_ref().get_property(property_name).get_raw()
}

#[no_mangle]
pub extern "C" fn get_property_by_var(
    this: *mut Variable,
    property_name: *mut Variable,
) -> *mut Variable {
    let this = Ptr::from_raw(this).unwrap();
    let property_name = Ptr::from_raw(property_name).unwrap();

    this.get_ref()
        .get_property(property_name.get_ref().to_string())
        .get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn function_call(
    this: *mut Variable,
    args: *mut *mut Variable,
    args_num: u32,
) -> *mut Variable {
    let this = Ptr::from_raw(this).unwrap();
    let args = from_raw_parts_mut(args, args_num as usize);

    this.get_ref().function_call(args).get_raw()
}
