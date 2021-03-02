struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    const SIZE: usize = 10_000;

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            buckets: vec![vec![]; Self::SIZE],
        }
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        let i = key as usize % Self::SIZE;
        if let Some(j) = (0..self.buckets[i].len()).find(|j| self.buckets[i][*j].0 == key) {
            self.buckets[i][j].1 = value;
        } else {
            self.buckets[i].push((key, value));
        }
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        let i = key as usize % Self::SIZE;
        self.buckets[i]
            .iter()
            .find(|(k, _)| k == &key)
            .unwrap_or(&(0, -1))
            .1
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        let i = key as usize % Self::SIZE;
        if let Some(j) = (0..self.buckets[i].len()).find(|j| self.buckets[i][*j].0 == key) {
            self.buckets[i].remove(j);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0706() {
        let mut obj = MyHashMap::new();
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        assert_eq!(obj.get(3), -1);
        obj.put(2, 1);
        assert_eq!(obj.get(2), 1);
        obj.remove(2);
        assert_eq!(obj.get(2), -1);
    }
}
