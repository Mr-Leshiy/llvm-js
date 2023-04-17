use crate::{pointer::Ptr, variable::Variable};

pub type FuncType = fn(*mut *mut Variable) -> *mut Variable;

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
        format!("Function, args_num: {0}", self.args_num)
    }

    pub fn call(&self, args: &mut [*mut Variable]) -> Ptr<Variable> {
        Ptr::from_raw((self.func)(args.as_mut_ptr())).expect("should be always valid")
    }
}
