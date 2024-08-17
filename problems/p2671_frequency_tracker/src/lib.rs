struct FrequencyTracker {
    count: Vec<i32>,
    freq: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
    fn new() -> Self {
        Self {
            count: vec![0; 100_001],
            freq: vec![0; 100_001],
        }
    }

    fn add(&mut self, number: i32) {
        self.freq[self.count[number as usize] as usize] -= 1;
        self.count[number as usize] += 1;
        self.freq[self.count[number as usize] as usize] += 1;
    }

    fn delete_one(&mut self, number: i32) {
        if self.count[number as usize] == 0 {
            return;
        }
        self.freq[self.count[number as usize] as usize] -= 1;
        self.count[number as usize] -= 1;
        self.freq[self.count[number as usize] as usize] += 1;
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        0 < self.freq[frequency as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2671() {
        let mut obj = FrequencyTracker::new();
        obj.add(3);
        obj.add(3);
        assert!(obj.has_frequency(2));

        let mut obj = FrequencyTracker::new();
        obj.add(1);
        obj.delete_one(1);
        assert!(!obj.has_frequency(1));

        let mut obj = FrequencyTracker::new();
        assert!(!obj.has_frequency(2));
        obj.add(3);
        assert!(obj.has_frequency(1));
    }
}
