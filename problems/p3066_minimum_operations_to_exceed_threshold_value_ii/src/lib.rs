use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(Reverse(num as usize));
        }
        for i in 0.. {
            if let Some(Reverse(num1)) = heap.pop() {
                if (k as usize) <= num1 {
                    return i;
                }
                if let Some(Reverse(num2)) = heap.pop() {
                    heap.push(Reverse(num1 * 2 + num2));
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3066() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 2);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 20), 4);
    }
}
