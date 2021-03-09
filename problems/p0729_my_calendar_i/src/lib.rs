struct MyCalendar {
    ranges: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        match self.ranges.binary_search_by(|a| a.0.cmp(&start)) {
            Ok(_) => false,
            Err(i) => {
                if (i == 0 || self.ranges[i - 1].1 <= start)
                    && (i == self.ranges.len() || end <= self.ranges[i].0)
                {
                    self.ranges.insert(i, (start, end));
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0729() {
        let mut obj = MyCalendar::new();
        assert!(obj.book(10, 20));
        assert!(!obj.book(15, 25));
        assert!(obj.book(20, 30));
        assert!(obj.book(1, 10));
        assert!(obj.book(50, 100));
    }
}
