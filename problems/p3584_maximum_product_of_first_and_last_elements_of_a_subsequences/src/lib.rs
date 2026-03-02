use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let m = m as usize;
        let mut queue = VecDeque::new();
        let mut max = std::i32::MIN;
        let mut min = std::i32::MAX;
        let mut result = std::i64::MIN;

        for i in 0..nums.len() {
            queue.push_back(nums[i]);
            if queue.len() == m {
                let v = queue.pop_front().unwrap();
                max = max.max(v);
                min = min.min(v);
            }
            if m <= i + 1 {
                result =
                    result.max(nums[i] as i64 * if nums[i] < 0 { min as i64 } else { max as i64 });
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3584() {
        assert_eq!(
            Solution::maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1),
            81
        );
        assert_eq!(Solution::maximum_product(vec![1, 3, -5, 5, 6, -4], 3), 20);
        assert_eq!(
            Solution::maximum_product(vec![2, -1, 2, -6, 5, 2, -5, 7], 2),
            35
        );
    }
}
