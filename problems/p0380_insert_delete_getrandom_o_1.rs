use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Default)]
struct RandomizedSet {
    map: HashMap<i32, usize>,
    values: Vec<Option<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.values.push(val.into());
            self.map.insert(val, self.values.len() - 1);
            true
        }
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            let pos = self.map.remove(&val).unwrap();
            self.values[pos] = None;
            true
        } else {
            false
        }
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let mut pos = rng.gen_range(0, self.values.len());
        while self.values[pos].is_none() {
            pos = rng.gen_range(0, self.values.len());
        }
        self.values[pos].unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0380() {
        let mut obj = RandomizedSet::new();
        assert!(obj.insert(1));
        assert!(!obj.remove(2));
        assert!(obj.insert(2));
        assert!([1, 2].contains(&obj.get_random()));
        assert!(obj.remove(1));
        assert!(!obj.insert(2));
        assert_eq!(2, obj.get_random());
    }
}
