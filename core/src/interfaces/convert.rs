use crate::variable::Variable;
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn convert_to_number(this: *mut Variable) -> *mut Variable {
    let this = NonNull::new(this).unwrap();

    let res = this.as_ref().to_number().into();
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_boolean(this: *mut Variable) -> *mut Variable {
    let this = NonNull::new(this).unwrap();

    let res = this.as_ref().to_boolean().into();
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_string(this: *mut Variable) -> *mut Variable {
    let this = NonNull::new(this).unwrap();

    let res = this.as_ref().to_string().into();
    Box::into_raw(Box::new(res))
}
