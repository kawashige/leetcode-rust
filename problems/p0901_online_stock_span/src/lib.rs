struct StockSpanner {
    stack: Vec<(i32, usize)>,
    day: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self {
            stack: vec![(10_001, 0)],
            day: 1,
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        while self.stack.last().unwrap().0 <= price {
            self.stack.pop();
        }
        self.stack.push((price, self.day));
        self.day += 1;
        (self.day - 1 - self.stack[self.stack.len() - 2].1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0901() {
        let mut obj = StockSpanner::new();
        assert_eq!(obj.next(100), 1);
        assert_eq!(obj.next(80), 1);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(70), 2);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(75), 4);
        assert_eq!(obj.next(85), 6);

        let mut obj = StockSpanner::new();
        assert_eq!(obj.next(28), 1);
        assert_eq!(obj.next(14), 1);
        assert_eq!(obj.next(28), 3);
        assert_eq!(obj.next(35), 4);
        assert_eq!(obj.next(46), 5);
        assert_eq!(obj.next(53), 6);
        assert_eq!(obj.next(66), 7);
        assert_eq!(obj.next(80), 8);
        assert_eq!(obj.next(87), 9);
        assert_eq!(obj.next(88), 10);
    }
}
