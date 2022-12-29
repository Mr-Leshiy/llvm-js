use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(Debug, Default)]
pub struct Set<V: Clone + Eq + Hash + Display> {
    stack: Vec<V>,
    hash_map: HashMap<V, u32>,
}

impl<V: Clone + Eq + Hash + Display + Debug> Set<V> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: V) -> u32 {
        match self.hash_map.entry(value.clone()) {
            Entry::Vacant(e) => {
                self.stack.push(value);
                e.insert(0);
                0_u32
            }
            Entry::Occupied(mut e) => {
                self.stack.push(value);
                e.insert(e.get() + 1);
                *e.get()
            }
        }
    }

    pub fn get(&self, value: &V) -> Option<u32> {
        self.hash_map.get(value).cloned()
    }

    pub fn remove_last_added(&mut self, mut size: usize) -> Vec<(V, u32)> {
        let mut res = Vec::new();
        while size > 0 {
            match self.stack.pop() {
                Some(value) => {
                    let counter = self
                        .hash_map
                        .get_mut(&value)
                        .unwrap_or_else(|| panic!("HashMap must contains key: {0}", value));
                    res.push((value.clone(), *counter));
                    if *counter == 0 {
                        self.hash_map
                            .remove(&value)
                            .unwrap_or_else(|| panic!("HashMap must contains key: {0}", value));
                    } else {
                        *counter -= 1;
                    }
                }
                None => break,
            }
            size -= 1;
        }
        res
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        assert!(self.hash_map.len() == self.stack.len());
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

        assert_eq!(set.remove_last_added(10), vec![]);
        assert_eq!(set.len(), 0);

        assert_eq!(set.insert(5), 0);
        assert_eq!(set.insert(5), 1);
        assert_eq!(set.insert(6), 0);

        assert_eq!(set.get(&5), Some(1));
        assert_eq!(set.get(&6), Some(0));
        assert_eq!(set.len(), 3);

        assert_eq!(set.remove_last_added(1), vec![(6, 0)]);

        assert_eq!(set.get(&5), Some(1));
        assert_eq!(set.get(&6), None);
        assert_eq!(set.len(), 2);

        assert_eq!(set.remove_last_added(1), vec![(5, 1)]);

        assert_eq!(set.get(&5), Some(0));
        assert_eq!(set.get(&6), None);
        assert_eq!(set.len(), 1);

        assert_eq!(set.remove_last_added(3), vec![(5, 0)]);

        assert_eq!(set.get(&5), None);
        assert_eq!(set.get(&6), None);
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn from_iter_test() {
        let vec = vec![1, 2, 3, 4, 5];

        let set: Set<i32> = vec.into_iter().collect();

        assert_eq!(set.len(), 5);
        assert_eq!(set.get(&1), Some(0));
        assert_eq!(set.get(&2), Some(0));
        assert_eq!(set.get(&3), Some(0));
        assert_eq!(set.get(&4), Some(0));
        assert_eq!(set.get(&5), Some(0));
    }
}
