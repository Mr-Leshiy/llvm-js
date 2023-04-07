use crate::variable::Variable;
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn convert_to_number(this: *mut Variable) -> *mut Variable {
    let this = NonNull::new(this).unwrap();

    Box::into_raw(Box::new(this.as_ref().to_number().into()))
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_boolean(this: *mut Variable) -> *mut Variable {
    let this = NonNull::new(this).unwrap();

    Box::into_raw(Box::new(this.as_ref().to_boolean().into()))
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_string(this: *mut Variable) -> *mut Variable {
    let this = NonNull::new(this).unwrap();

    Box::into_raw(Box::new(this.as_ref().to_string().into()))
}
