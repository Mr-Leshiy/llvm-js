use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error<K> {
    #[error("map already contains value by the corresponding key: {0}")]
    AlreadyKnownKey(K),
    #[error("map does not contain key: {0}")]
    UnknownKey(K),
}

#[derive(Clone, Default)]
pub struct Map<K: Clone + Eq + Hash + Display, V> {
    stack: Vec<K>,
    hash_map: HashMap<K, V>,
}

impl<K: Clone + Eq + Hash + Display + Debug, V> Map<K, V> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), Error<K>> {
        if let Entry::Vacant(e) = self.hash_map.entry(key.clone()) {
            self.stack.push(key);
            e.insert(value);
            Ok(())
        } else {
            Err(Error::AlreadyKnownKey(key))
        }
    }

    pub fn update(&mut self, key: K, value: V) -> Result<(), Error<K>> {
        if let Entry::Occupied(mut e) = self.hash_map.entry(key.clone()) {
            e.insert(value);
            Ok(())
        } else {
            Err(Error::UnknownKey(key))
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.hash_map.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.hash_map.get(key)
    }

    pub fn remove_last_added(&mut self, mut size: usize) {
        while size > 0 {
            match self.stack.pop() {
                Some(key) => self.hash_map.remove(&key).unwrap_or_else(|| {
                    panic!("HashMap must contains the corresponding key: {0}", key)
                }),
                None => break,
            };
            size -= 1;
        }
    }

    pub fn len(&self) -> usize {
        assert!(self.hash_map.len() == self.stack.len());
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        assert!(self.hash_map.len() == self.stack.len());
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let mut map = Map::new();

        map.remove_last_added(10);
        assert_eq!(map.len(), 0);

        map.insert(5, "value").unwrap();
        assert_eq!(map.insert(5, "value"), Err(Error::AlreadyKnownKey(5)));

        map.insert(6, "value2").unwrap();

        assert_eq!(map.get(&5), Some(&"value"));
        assert!(map.contains(&5));
        assert_eq!(map.get(&6), Some(&"value2"));
        assert!(map.contains(&6));
        assert_eq!(map.len(), 2);

        map.remove_last_added(1);

        assert_eq!(map.get(&5), Some(&"value"));
        assert!(map.contains(&5));
        assert_eq!(map.get(&6), None);
        assert!(!map.contains(&6));
        assert_eq!(map.len(), 1);

        map.update(5, "new value").unwrap();
        assert_eq!(map.update(6, "new_value"), Err(Error::UnknownKey(6)));
        assert_eq!(map.len(), 1);

        map.remove_last_added(3);

        assert_eq!(map.get(&5), None);
        assert!(!map.contains(&5));
        assert_eq!(map.get(&6), None);
        assert!(!map.contains(&6));
        assert_eq!(map.len(), 0);
    }
}
