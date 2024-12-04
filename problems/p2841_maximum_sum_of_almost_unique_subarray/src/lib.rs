use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let mut count = HashMap::new();
        let mut distinct_numbers = 0;
        let mut sum = 0;
        let mut result = 0;
        let k = k as usize;

        for i in 0..nums.len() {
            *count.entry(nums[i]).or_insert(0) += 1;
            sum += nums[i] as i64;
            if count[&nums[i]] == 1 {
                distinct_numbers += 1;
            }
            if k <= i {
                *count.entry(nums[i - k]).or_insert(0) -= 1;
                sum -= nums[i - k] as i64;
                if count[&nums[i - k]] == 0 {
                    distinct_numbers -= 1;
                }
            }
            if k - 1 <= i && m <= distinct_numbers {
                result = result.max(sum);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2841() {
        assert_eq!(Solution::max_sum(vec![2, 6, 7, 3, 1, 7], 3, 4), 18);
        assert_eq!(Solution::max_sum(vec![5, 9, 9, 2, 4, 5, 4], 1, 3), 23);
        assert_eq!(Solution::max_sum(vec![1, 2, 1, 2, 1, 2, 1], 3, 3), 0);
    }
}
