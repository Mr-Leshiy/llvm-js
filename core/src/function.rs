use crate::{ptr::RawPtr, variable::Variable};

pub type FuncType = extern "C" fn(*mut *mut Variable) -> *mut Variable;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    func: FuncType,
    args_num: u32,
}

impl Function {
    pub fn new(func: FuncType, args_num: u32) -> Self {
        Self { func, args_num }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        format!(
            "Function, ptr: {0:?}, args_num: {1}",
            self.func, self.args_num
        )
    }

    pub fn call(&self, args: &mut Vec<*mut Variable>) -> RawPtr<Variable> {
        while args.len() < self.args_num as usize {
            args.push(RawPtr::allocate(Variable::Undefined).get_raw());
        }
        let res = (self.func)(args.as_mut_ptr());
        RawPtr::from_raw(res).expect("should be always valid")
    }
}
