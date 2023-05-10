use crate::{ptr::Ptr, variable::Variable};

#[no_mangle]
pub extern "C" fn allocate() -> *mut Variable {
    let val = Variable::Undefined;
    Ptr::allocate(val).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(_this: *mut Variable) {}
