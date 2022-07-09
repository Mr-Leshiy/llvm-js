use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error<V> {
    #[error("set already contains value: {0}")]
    AlreadyKnownValue(V),
}

#[derive(Default)]
pub struct Set<V: Clone + Eq + Hash + Display> {
    stack: Vec<V>,
    hash_set: HashSet<V>,
}

impl<V: Clone + Eq + Hash + Display + Debug> Set<V> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            hash_set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, value: V) -> Result<(), Error<V>> {
        if !self.hash_set.contains(&value) {
            self.stack.push(value.clone());
            self.hash_set
                .insert(value.clone())
                .then(|| ())
                .unwrap_or_else(|| panic!("HashSet should not contains value: {0}", value));
            Ok(())
        } else {
            Err(Error::AlreadyKnownValue(value))
        }
    }

    pub fn contains(&self, value: &V) -> bool {
        self.hash_set.contains(value)
    }

    pub fn remove_last_added(&mut self, mut size: usize) {
        while size > 0 {
            match self.stack.pop() {
                Some(value) => self
                    .hash_set
                    .remove(&value)
                    .then(|| ())
                    .unwrap_or_else(|| panic!("HashSet must contains value: {0}", value)),
                None => break,
            };
            size -= 1;
        }
    }

    pub fn len(&self) -> usize {
        assert!(self.hash_set.len() == self.stack.len());
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        assert!(self.hash_set.len() == self.stack.len());
        self.stack.is_empty()
    }
}

impl<V: Clone + Eq + Hash + Display + Debug> FromIterator<V> for Set<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut set = Set::new();
        for el in iter.into_iter() {
            let _ = set.insert(el);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_test() {
        let mut set = Set::new();

        set.remove_last_added(10);
        assert_eq!(set.len(), 0);

        set.insert(5).unwrap();
        assert_eq!(set.insert(5), Err(Error::AlreadyKnownValue(5)));

        set.insert(6).unwrap();

        assert!(set.contains(&5));
        assert!(set.contains(&6));
        assert_eq!(set.len(), 2);

        set.remove_last_added(1);

        assert!(set.contains(&5));
        assert!(!set.contains(&6));
        assert_eq!(set.len(), 1);

        set.remove_last_added(3);

        assert!(!set.contains(&5));
        assert!(!set.contains(&6));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn from_iter_test() {
        let vec = vec![1, 2, 3, 4, 5];

        let set: Set<i32> = vec.into_iter().collect();

        assert_eq!(set.len(), 5);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&5));
    }
}
