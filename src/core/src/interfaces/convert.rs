use crate::{ptr::RawPtr, variable::VariableValue};

#[no_mangle]
pub extern "C" fn convert_to_number(this: *mut VariableValue) -> *mut VariableValue {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(VariableValue::from(this.to_number())).get_raw()
}

#[no_mangle]
pub extern "C" fn convert_to_boolean(this: *mut VariableValue) -> *mut VariableValue {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(VariableValue::from(this.to_boolean())).get_raw()
}

#[no_mangle]
pub extern "C" fn convert_to_string(this: *mut VariableValue) -> *mut VariableValue {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(VariableValue::from(this.to_string())).get_raw()
}
