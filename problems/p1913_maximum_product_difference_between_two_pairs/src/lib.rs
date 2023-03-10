pub struct Solution {}

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len() - 2] * nums[nums.len() - 1] - nums[0] * nums[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1913() {
        assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
        assert_eq!(
            Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]),
            64
        );
    }
}
