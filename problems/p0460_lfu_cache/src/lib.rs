use std::collections::VecDeque;

struct LFUCache {
    values: Vec<(i32, usize)>,
    frequencies: Vec<VecDeque<(i32, usize)>>,
    count: usize,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            values: vec![(-1, 0); 100_001],
            frequencies: vec![VecDeque::new(), VecDeque::new()],
            count: 0,
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        if self.values[key as usize].0 != -1 {
            self.values[key as usize].1 += 1;
            if self.frequencies.len() - 1 < self.values[key as usize].1 {
                self.frequencies.push(VecDeque::new());
            }
            self.frequencies[self.values[key as usize].1]
                .push_back((key, self.values[key as usize].1));
        }
        self.values[key as usize].0
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if self.values[key as usize].0 == -1 {
            if self.count == self.capacity {
                let mut found = false;
                for j in 1.. {
                    if found {
                        break;
                    }
                    while let Some((del_key, del_count)) = self.frequencies[j].pop_front() {
                        if self.values[del_key as usize].1 == del_count {
                            self.values[del_key as usize] = (-1, 0);
                            found = true;
                            break;
                        }
                    }
                }
            } else {
                self.count += 1;
            }
            self.values[key as usize] = (value, 1);
            self.frequencies[1].push_back((key, 1));
        } else {
            self.values[key as usize].0 = value;
            self.values[key as usize].1 += 1;
            if self.frequencies.len() - 1 < self.values[key as usize].1 {
                self.frequencies.push(VecDeque::new());
            }
            self.frequencies[self.values[key as usize].1]
                .push_back((key, self.values[key as usize].1));
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
