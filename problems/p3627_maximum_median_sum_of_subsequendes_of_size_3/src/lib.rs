pub struct Solution {}

impl Solution {
    pub fn maximum_median_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = 0;
        for i in 0..nums.len() / 3 {
            sum += nums[nums.len() - 2 - 2 * i] as i64;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3627() {
        assert_eq!(Solution::maximum_median_sum(vec![2, 1, 3, 2, 1, 3]), 5);
        assert_eq!(Solution::maximum_median_sum(vec![1, 1, 10, 10, 10, 10]), 20);
    }
}
