pub struct Solution {}

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut sum = nums[0];

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                max_sum = max_sum.max(sum);
                sum = 0;
            }
            sum += nums[i];
        }

        max_sum.max(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1800() {
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
        assert_eq!(
            Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]),
            33
        );
    }
}
