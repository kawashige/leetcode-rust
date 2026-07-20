use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, s: String) -> i64 {
        let mut one_count = 0;
        let mut heap = BinaryHeap::new();

        for i in (0..s.len()).rev() {
            if s.as_bytes()[i] == b'1' {
                one_count += 1;
            }
            heap.push(Reverse(nums[i]));
            if one_count < heap.len() {
                heap.pop();
            }
        }

        let mut result = 0;
        while let Some(Reverse(n)) = heap.pop() {
            result += n as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3781() {
        assert_eq!(
            Solution::maximum_score(vec![2, 1, 5, 2, 3], "01010".to_string()),
            7
        );
        assert_eq!(
            Solution::maximum_score(vec![4, 7, 2, 9], "0000".to_string()),
            0
        );
    }
}

fn main() {}
