use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut result = 0;
        let mut aligned = 0;

        for flip in flips {
            heap.push(Reverse(flip));
            while let Some(Reverse(x)) = heap.peek() {
                if x == &(aligned + 1) {
                    aligned += 1;
                    heap.pop();
                } else {
                    break;
                }
            }

            if heap.is_empty() {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1375() {
        assert_eq!(Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]), 2);
        assert_eq!(Solution::num_times_all_blue(vec![4, 1, 2, 3]), 1);
    }
}
