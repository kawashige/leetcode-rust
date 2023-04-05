pub struct Solution {}

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut min = nums[0] as usize;
        let mut sum = nums[0] as usize;

        for i in 1..nums.len() {
            sum += nums[i] as usize;
            min = min.max((sum + i) / (i + 1));
        }

        min as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2439() {
        assert_eq!(
            Solution::minimize_array_value(vec![13, 13, 20, 0, 8, 9, 9]),
            16
        );
        assert_eq!(Solution::minimize_array_value(vec![3, 7, 1, 6]), 5);
        assert_eq!(Solution::minimize_array_value(vec![10, 1]), 10);
    }
}
