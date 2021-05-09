use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut heap = BinaryHeap::new();
        let mut sum: i64 = 0;
        let len = target.len() as i64;
        for n in target {
            sum += n as i64;
            if n != 1 {
                heap.push(n as i64);
            }
        }

        while let Some(n) = heap.pop() {
            if let Some(next) = heap.peek() {
                let mut x = n;

                while 1 < x && *next <= x {
                    let tmp = x;
                    x = 2 * x - sum;
                    if x <= 0 {
                        return false;
                    }
                    sum = tmp;
                }
                if x != 1 {
                    heap.push(x);
                }
            } else {
                return len > 1 && (n - 1) % (len - 1) == 0;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day09() {
        assert!(Solution::is_possible(vec![1, 1000000000]));
        assert!(!Solution::is_possible(vec![1, 1, 1, 2]));
        assert!(Solution::is_possible(vec![8, 5]));
    }
}
