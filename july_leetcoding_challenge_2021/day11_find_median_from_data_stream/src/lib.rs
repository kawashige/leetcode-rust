use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct MedianFinder {
    first_half: BinaryHeap<i32>,
    second_half: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        match (self.first_half.peek(), self.second_half.peek()) {
            (Some(x), _) => {
                if &num <= x {
                    self.first_half.push(num);
                } else {
                    self.second_half.push(Reverse(num));
                }

                while self.first_half.len() + 1 < self.second_half.len() {
                    if let Some(Reverse(x)) = self.second_half.pop() {
                        self.first_half.push(x);
                    }
                }
                while self.second_half.len() + 1 < self.first_half.len() {
                    if let Some(x) = self.first_half.pop() {
                        self.second_half.push(Reverse(x));
                    }
                }
            }
            (None, _) => self.first_half.push(num),
        }
    }

    fn find_median(&self) -> f64 {
        match (self.first_half.peek(), self.second_half.peek()) {
            (Some(x), Some(Reverse(y))) => {
                if self.first_half.len() == self.second_half.len() {
                    (*x as f64 + *y as f64) / 2.0
                } else if self.first_half.len() > self.second_half.len() {
                    *x as f64
                } else {
                    *y as f64
                }
            }
            (Some(x), None) => *x as f64,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day11() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        assert_eq!(obj.find_median(), 1.5);
        obj.add_num(3);
        assert_eq!(obj.find_median(), 2.0);
        obj.add_num(2);
        assert_eq!(obj.find_median(), 2.0);
    }
}
