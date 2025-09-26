pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] += sum[i] + nums[i];
            result += sum[i + 1] - sum[i - (nums[i] as usize).min(i)];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3427() {
        assert_eq!(Solution::subarray_sum(vec![2, 3, 1]), 11);
        assert_eq!(Solution::subarray_sum(vec![3, 1, 1, 2]), 13);
    }
}
