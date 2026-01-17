use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn find_max_sum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        let mut tmp: Vec<(i32, usize)> = Vec::new();
        let mut nums1 = nums1.into_iter().zip(0..).collect::<Vec<_>>();
        nums1.sort_unstable_by_key(|(x, _)| *x);
        let mut result = vec![0; nums1.len()];
        let mut sum = 0;

        for i in 0..nums1.len() {
            while !tmp.is_empty() && (*tmp.last().unwrap()).0 < nums1[i].0 {
                let num = nums2[tmp.pop().unwrap().1] as i64;
                sum += num;
                heap.push(Reverse(num));
            }
            while k < heap.len() {
                if let Some(Reverse(num)) = heap.pop() {
                    sum -= num;
                }
            }
            tmp.push(nums1[i].clone());

            result[nums1[i].1] = sum;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3478() {
        assert_eq!(
            Solution::find_max_sum(vec![4, 2, 1, 5, 3], vec![10, 20, 30, 40, 50], 2),
            vec![80, 30, 0, 80, 50]
        );
        assert_eq!(
            Solution::find_max_sum(vec![2, 2, 2, 2], vec![3, 1, 2, 3], 1),
            vec![0, 0, 0, 0]
        );
    }
}
