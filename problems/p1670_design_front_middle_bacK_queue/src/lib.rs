use std::collections::VecDeque;

#[derive(Default)]
struct FrontMiddleBackQueue {
    head: VecDeque<i32>,
    tail: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push_front(&mut self, val: i32) {
        self.head.push_front(val);
        self.rebalance();
    }

    fn push_middle(&mut self, val: i32) {
        self.head.push_back(val);
        self.rebalance();
    }

    fn push_back(&mut self, val: i32) {
        self.tail.push_back(val);
        self.rebalance();
    }

    fn pop_front(&mut self) -> i32 {
        let val = if self.head.is_empty() {
            self.tail.pop_front().unwrap_or(-1)
        } else {
            self.head.pop_front().unwrap_or(-1)
        };
        self.rebalance();
        val
    }

    fn pop_middle(&mut self) -> i32 {
        let val = if self.tail.is_empty() && self.head.is_empty() {
            -1
        } else if self.tail.len() <= self.head.len() {
            self.head.pop_back().unwrap()
        } else {
            self.tail.pop_front().unwrap()
        };
        self.rebalance();
        val
    }

    fn pop_back(&mut self) -> i32 {
        let val = if self.tail.is_empty() {
            self.head.pop_back().unwrap_or(-1)
        } else {
            self.tail.pop_back().unwrap_or(-1)
        };
        self.rebalance();
        val
    }

    fn rebalance(&mut self) {
        while self.head.len() + 1 < self.tail.len() {
            self.head.push_back(self.tail.pop_front().unwrap());
        }
        while self.head.len() > self.tail.len() {
            self.tail.push_front(self.head.pop_back().unwrap());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1670() {
        let mut obj = FrontMiddleBackQueue::new();
        obj.push_front(1);
        obj.push_back(2);
        obj.push_middle(3);
        obj.push_middle(4);
        assert_eq!(obj.pop_front(), 1);
        assert_eq!(obj.pop_middle(), 3);
        assert_eq!(obj.pop_middle(), 4);
        assert_eq!(obj.pop_back(), 2);
        assert_eq!(obj.pop_front(), -1);
    }
}
