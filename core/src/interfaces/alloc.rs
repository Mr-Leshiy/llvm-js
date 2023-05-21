use crate::{ptr::RawPtr, variable::Variable};

#[no_mangle]
pub extern "C" fn allocate() -> *mut Variable {
    let val = Variable::Undefined;
    RawPtr::allocate(val).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(_this: *mut Variable) {}
