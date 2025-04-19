use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        for i in 0..nums.len() {
            sum += nums[i] as i64;
            heap.push((Reverse(nums[i]), Reverse(i)));
        }
        let mut marked = vec![false; nums.len()];
        let mut result = vec![0; queries.len()];

        for i in 0..queries.len() {
            if !marked[queries[i][0] as usize] {
                marked[queries[i][0] as usize] = true;
                sum -= nums[queries[i][0] as usize] as i64;
            }
            for _ in 0..queries[i][1] {
                while let Some((Reverse(n), Reverse(j))) = heap.pop() {
                    if !marked[j] {
                        marked[j] = true;
                        sum -= n as i64;
                        break;
                    }
                }
            }
            result[i] = sum;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3080() {
        assert_eq!(
            Solution::unmarked_sum_array(
                vec![1, 2, 2, 1, 2, 3, 1],
                vec![vec![1, 2], vec![3, 3], vec![4, 2]]
            ),
            vec![8, 3, 0]
        );
        assert_eq!(
            Solution::unmarked_sum_array(vec![1, 4, 2, 3], vec![vec![0, 1]]),
            vec![7]
        );
    }
}
