use std::ops::Deref;

pub struct Context(inkwell::context::Context);

impl Deref for Context {
    type Target = inkwell::context::Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Context {
    pub fn new() -> Self {
        Self(inkwell::context::Context::create())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
