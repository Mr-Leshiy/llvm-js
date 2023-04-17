use crate::{pointer::Ptr, variable::Variable};
use std::ffi::{c_char, CStr};

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
pub unsafe extern "C" fn add_property_by_var(
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

// #[no_mangle]
// pub unsafe extern "C" fn get_property_by_str(
//     this: *mut Variable,
//     property_name: *const c_char,
// ) -> *mut Variable {
//     let mut this = Ptr::from_raw(this).unwrap();
//     let property_name = CStr::from_ptr(property_name).to_str().unwrap().to_string();


// }
