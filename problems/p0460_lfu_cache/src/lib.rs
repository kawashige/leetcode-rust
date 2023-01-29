struct LFUCache {
    values: Vec<i32>,
    cache: Vec<(usize, i32, i32)>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            values: vec![-1; 100_001],
            cache: Vec::with_capacity(capacity as usize),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        println!(
            "cache: {:?}, values: {:?}",
            self.cache,
            (0..5).map(|i| self.values[i]).collect::<Vec<_>>()
        );
        if self.capacity == 0 {
            return -1;
        }
        if self.values[key as usize] != -1 {
            let val = self.cache[self.values[key as usize] as usize].2;
            self.cache[self.values[key as usize] as usize].0 += 1;
            self.rearrange(self.values[key as usize] as usize);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        println!(
            "cache: {:?}, values: {:?}",
            self.cache,
            (0..5).map(|i| self.values[i]).collect::<Vec<_>>()
        );
        if self.capacity == 0 {
            return;
        }
        if self.values[key as usize] == -1 {
            if self.cache.len() == self.capacity {
                let (_, del_key, _) = self.cache.pop().unwrap();
                self.values[del_key as usize] = -1;
            }
            self.cache.push((1, key, value));
            self.values[key as usize] = self.cache.len() as i32 - 1;
        } else {
            self.cache[self.values[key as usize] as usize].0 += 1;
            self.cache[self.values[key as usize] as usize].2 = value;
        }
        self.rearrange(self.values[key as usize] as usize);
    }

    fn rearrange(&mut self, start: usize) {
        for i in (0..start).rev() {
            if self.cache[i + 1].0 < self.cache[i].0 {
                break;
            }
            self.values[self.cache[i].1 as usize] = i as i32 + 1;
            self.values[self.cache[i + 1].1 as usize] = i as i32;
            self.cache.swap(i, i + 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0460() {
        let mut obj = LFUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        obj.put(3, 3);
        assert_eq!(obj.get(2), -1);
        assert_eq!(obj.get(3), 3);
        obj.put(4, 4);
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), 4);

        let mut obj = LFUCache::new(2);
        obj.put(2, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(2), 2);
        obj.put(1, 1);
        obj.put(4, 1);
        assert_eq!(obj.get(2), 2);

        let mut obj = LFUCache::new(3);
        obj.put(1, 1);
        obj.put(2, 2);
        obj.put(3, 3);
        obj.put(4, 4);
        assert_eq!(obj.get(4), 4);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(2), 2);
        assert_eq!(obj.get(1), -1);
        obj.put(5, 5);
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(2), 2);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), -1);
        assert_eq!(obj.get(5), 5);
    }
}
