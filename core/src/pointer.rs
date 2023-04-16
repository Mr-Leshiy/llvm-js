use std::marker::PhantomData;

#[derive(Debug)]
pub struct Ptr<T, A = ()> {
    raw: *mut T,
    _a: PhantomData<A>,
}

impl<T> Ptr<T> {
    pub fn new(raw: *mut T) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw,
                _a: PhantomData::default(),
            })
        }
    }

    pub fn get_raw(&self) -> *mut T {
        self.raw
    }

    pub fn get_ref(&self) -> &T {
        unsafe { &*self.raw }
    }

    pub fn get_mut_ref(&mut self) -> &mut T {
        unsafe { &mut *self.raw }
    }
}

impl<T> Clone for Ptr<T, ()> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw,
            _a: self._a,
        }
    }
}

impl<T: PartialEq, A> PartialEq for Ptr<T, A> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.raw) == (*other.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test() {
        assert_eq!(Ptr::new(&mut 5).unwrap(), Ptr::new(&mut 5).unwrap());
        assert_ne!(Ptr::new(&mut 5).unwrap(), Ptr::new(&mut 10).unwrap());
    }

    #[test]
    fn basic_test() {
        let mut ptr = Ptr::new(&mut 5).unwrap();

        assert_eq!(ptr.get_ref(), &5);
        assert_eq!(ptr.get_mut_ref(), &mut 5);
        unsafe {
            assert_eq!(*ptr.get_raw(), 5);
        }
    }
}
