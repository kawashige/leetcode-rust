pub struct Solution {}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let result = nums[0];
        let mut nums = nums[1..].to_vec();
        nums.sort_unstable();
        result + nums[0] + nums[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3010() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 12]), 6);
        assert_eq!(Solution::minimum_cost(vec![5, 4, 3]), 12);
        assert_eq!(Solution::minimum_cost(vec![10, 3, 1, 1]), 12);
    }
}
