use std::collections::VecDeque;

#[derive(Default)]
struct RecentCounter {
    calls: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Default::default()
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.calls.push_back(t);
        while self.calls[0] < t - 3000 {
            self.calls.pop_front();
        }
        self.calls.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        let mut obj = RecentCounter::new();
        assert_eq!(1, obj.ping(1));
        assert_eq!(2, obj.ping(100));
        assert_eq!(3, obj.ping(3001));
        assert_eq!(3, obj.ping(3002));
    }
}
