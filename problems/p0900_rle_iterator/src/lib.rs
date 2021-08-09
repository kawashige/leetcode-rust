use std::collections::VecDeque;

struct RLEIterator {
    values: VecDeque<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self {
            values: (0..encoding.len())
                .step_by(2)
                .filter(|i| encoding[*i] != 0)
                .map(|i| (encoding[i], encoding[i + 1]))
                .collect(),
        }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut rest = n;
        while let Some((count, val)) = self.values.pop_front() {
            if rest == count {
                return val;
            } else if rest < count {
                self.values.push_front((count - rest, val));
                return val;
            } else {
                rest -= count;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0900() {
        let mut obj = RLEIterator::new(vec![784, 303, 477, 583, 909, 505]);
        assert_eq!(obj.next(130), 303);
        assert_eq!(obj.next(333), 303);
        assert_eq!(obj.next(238), 303);
        assert_eq!(obj.next(87), 583);
        assert_eq!(obj.next(301), 583);
        assert_eq!(obj.next(276), 505);

        let mut obj = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
        assert_eq!(obj.next(2), 8);
        assert_eq!(obj.next(1), 8);
        assert_eq!(obj.next(1), 5);
        assert_eq!(obj.next(2), -1);
    }
}
