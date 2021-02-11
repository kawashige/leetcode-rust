pub struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = nums[..(k as usize)].iter().sum::<i32>();
        let mut max_sum = sum;
        for i in (k as usize)..nums.len() {
            sum += nums[i] - nums[i - k as usize];
            max_sum = std::cmp::max(max_sum, sum);
        }
        max_sum as f64 / k as f64
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0643() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
        assert_eq!(Solution::find_max_average(vec![1], 1), 1.0);
    }
}
