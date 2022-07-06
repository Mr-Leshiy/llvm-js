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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variables_storage_test() {
        let mut map = Set::new();

        map.remove_last_added(10);
        assert_eq!(map.len(), 0);

        map.insert(5).unwrap();
        assert_eq!(map.insert(5), Err(Error::AlreadyKnownValue(5)));

        map.insert(6).unwrap();

        assert!(map.contains(&5));
        assert!(map.contains(&6));
        assert_eq!(map.len(), 2);

        map.remove_last_added(1);

        assert!(map.contains(&5));
        assert!(!map.contains(&6));
        assert_eq!(map.len(), 1);

        map.remove_last_added(3);

        assert!(!map.contains(&5));
        assert!(!map.contains(&6));
        assert_eq!(map.len(), 0);
    }
}
