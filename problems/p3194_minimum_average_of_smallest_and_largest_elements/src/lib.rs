pub struct Solution {}

impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums = nums;
        nums.sort_unstable();
        (0..nums.len() / 2)
            .map(|i| nums[i] + nums[nums.len() - 1 - i])
            .min()
            .unwrap() as f64
            / 2.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3194() {
        assert_eq!(
            Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]),
            5.5
        );
        assert_eq!(Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5);
        assert_eq!(Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
}
