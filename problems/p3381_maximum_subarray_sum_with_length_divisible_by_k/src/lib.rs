pub struct Solution {}

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut minimum_sum = vec![std::i64::MAX; k];
        let mut sum = 0;
        let mut result = std::i64::MIN;

        for i in 0..nums.len() {
            sum += nums[i] as i64;
            if k + 1 <= i {
                result = result.max(sum - minimum_sum[i % k]);
            }
            if (i + 1) % k == 0 {
                result = result.max(sum);
            }
            minimum_sum[i % k] = minimum_sum[i % k].min(sum);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3381() {
        assert_eq!(Solution::max_subarray_sum(vec![1, 2], 1), 3);
        assert_eq!(Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4), -10);
        assert_eq!(Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2), 4);
    }
}
