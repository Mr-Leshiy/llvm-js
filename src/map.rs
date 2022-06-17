use std::{collections::HashMap, fmt::Display, hash::Hash};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error<K> {
    #[error("map already contains value by the corresponding key: {0}")]
    AlreadyKnownKey(K),
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
        if !self.hash_map.contains_key(&key) {
            self.stack.push(key.clone());
            self.hash_map.insert(key, value);
            Ok(())
        } else {
            Err(Error::AlreadyKnownKey(key))
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.hash_map.get(key)
    }

    pub fn remove_last_added(&mut self, mut size: usize) {
        while size > 0 {
            match self.stack.pop() {
                Some(key) => self.hash_map.remove(&key).expect(
                    format!("HashMap must contains the corresponding key: {0}", key).as_str(),
                ),
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

        map.remove_last_added(3);

        assert_eq!(map.get(&5), None);
        assert_eq!(map.get(&6), None);
        assert_eq!(map.len(), 0);
    }
}
