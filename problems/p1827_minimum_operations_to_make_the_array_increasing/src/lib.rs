pub struct Solution {}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                count += nums[i - 1] - nums[i] + 1;
                nums[i] = nums[i - 1] + 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1827() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
        assert_eq!(Solution::min_operations(vec![8]), 0);
    }
}
