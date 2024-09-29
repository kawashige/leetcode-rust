use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Default)]
struct AllOne {
    keys: HashMap<String, usize>,
    max: BinaryHeap<(usize, String)>,
    min: BinaryHeap<(Reverse<usize>, String)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Default::default()
    }

    fn inc(&mut self, key: String) {
        let c = *self.keys.get(&key).unwrap_or(&0);
        self.keys.insert(key.to_string(), c + 1);
        self.max.push((c + 1, key.to_string()));
        self.min.push((Reverse(c + 1), key));
    }

    fn dec(&mut self, key: String) {
        let c = *self.keys.get(&key).unwrap();
        if c == 1 {
            self.keys.remove(&key);
        } else {
            self.keys.insert(key.to_string(), c - 1);
            self.max.push((c - 1, key.to_string()));
            self.min.push((Reverse(c - 1), key));
        }
    }

    fn get_max_key(&mut self) -> String {
        println!(
            "keys: {:?}, min: {:?}, max: {:?}",
            self.keys, self.min, self.max
        );
        while let Some((c, k)) = self.max.pop() {
            if self.keys.get(&k).unwrap_or(&0) == &c {
                self.max.push((c, k.to_string()));
                return k;
            }
        }
        Default::default()
    }

    fn get_min_key(&mut self) -> String {
        println!(
            "keys: {:?}, min: {:?}, max: {:?}",
            self.keys, self.min, self.max
        );
        while let Some((Reverse(c), k)) = self.min.pop() {
            if self.keys.get(&k).unwrap_or(&0) == &c {
                self.min.push((Reverse(c), k.to_string()));
                return k;
            }
        }
        Default::default()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0432() {
        let mut obj = AllOne::new();
        obj.inc("hello".to_string());
        obj.inc("hello".to_string());
        assert_eq!(obj.get_max_key(), "hello".to_string());
        assert_eq!(obj.get_min_key(), "hello".to_string());
        obj.inc("leet".to_string());
        assert_eq!(obj.get_max_key(), "hello".to_string());
        assert_eq!(obj.get_min_key(), "leet".to_string());
    }
}
