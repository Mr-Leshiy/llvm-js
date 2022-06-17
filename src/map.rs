use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
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

pub struct Map<K, V> {
    stack: Vec<K>,
    hash_map: HashMap<K, V>,
}

impl<K: Clone + Eq + Hash + Display, V> Map<K, V> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variables_storage_test() {
        let mut map = Map::new();

        map.remove_last_added(10);
        assert_eq!(map.len(), 0);

        map.insert(5, "value").unwrap();
        assert_eq!(map.insert(5, "value"), Err(Error::AlreadyKnownKey(5)));

        map.insert(6, "value2").unwrap();

        assert_eq!(map.get(&5), Some(&"value"));
        assert_eq!(map.get(&6), Some(&"value2"));
        assert_eq!(map.len(), 2);

        map.remove_last_added(1);

        assert_eq!(map.get(&5), Some(&"value"));
        assert_eq!(map.get(&6), None);
        assert_eq!(map.len(), 1);

        map.update(5, "new value").unwrap();
        assert_eq!(map.update(6, "new_value"), Err(Error::UnknownKey(6)));
        assert_eq!(map.len(), 1);

        map.remove_last_added(3);

        assert_eq!(map.get(&5), None);
        assert_eq!(map.get(&6), None);
        assert_eq!(map.len(), 0);
    }
}
