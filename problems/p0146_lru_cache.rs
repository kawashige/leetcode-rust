use std::collections::HashMap;

#[derive(Default)]
struct LRUCache {
    cache: HashMap<i32, i32>,
    keys: Vec<i32>,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            ..Default::default()
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        println!("cache: {:?}, keys: {:?}", self.cache, self.keys);
        match self.cache.get(&key) {
            Some(n) => {
                if self.keys.contains(&key) {
                    self.keys = self
                        .keys
                        .clone()
                        .into_iter()
                        .filter(|i| i != &key)
                        .collect();
                }
                self.keys.push(key);
                *n
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        println!("cache: {:?}, keys: {:?}", self.cache, self.keys);
        self.cache.insert(key, value);
        if self.keys.contains(&key) {
            self.keys = self
                .keys
                .clone()
                .into_iter()
                .filter(|i| i != &key)
                .collect();
        }
        self.keys.push(key);
        if self.cache.len() > self.capacity as usize {
            let removed = self.keys.remove(0);
            self.cache.remove(&removed);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0146() {
        let mut obj = LRUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(1, obj.get(1));
        obj.put(3, 3);
        assert_eq!(-1, obj.get(2));
        obj.put(4, 4);
        assert_eq!(-1, obj.get(1));
        assert_eq!(3, obj.get(3));
        assert_eq!(4, obj.get(4));

        let mut obj = LRUCache::new(2);
        obj.put(2, 1);
        obj.put(3, 2);
        assert_eq!(2, obj.get(3));
        assert_eq!(1, obj.get(2));
        obj.put(4, 3);
        assert_eq!(1, obj.get(2));
        assert_eq!(-1, obj.get(3));
        assert_eq!(3, obj.get(4));

        let mut obj = LRUCache::new(3);
        obj.put(1, 1);
        obj.put(2, 2);
        obj.put(3, 3);
        obj.put(4, 4);
        assert_eq!(4, obj.get(4));
        assert_eq!(3, obj.get(3));
        assert_eq!(2, obj.get(2));
        assert_eq!(-1, obj.get(1));
        obj.put(5, 5);
        assert_eq!(-1, obj.get(1));
        assert_eq!(2, obj.get(2));
        assert_eq!(3, obj.get(3));
        assert_eq!(-1, obj.get(4));
        assert_eq!(5, obj.get(5));

        let mut obj = LRUCache::new(2);
        obj.put(2, 1);
        obj.put(1, 1);
        assert_eq!(1, obj.get(2));
        obj.put(4, 1);
        assert_eq!(-1, obj.get(1));
        assert_eq!(1, obj.get(2));

        let mut obj = LRUCache::new(2);
        obj.put(2, 1);
        obj.put(1, 1);
        obj.put(2, 3);
        obj.put(4, 1);
        assert_eq!(-1, obj.get(1));
        assert_eq!(3, obj.get(2));
    }
}
