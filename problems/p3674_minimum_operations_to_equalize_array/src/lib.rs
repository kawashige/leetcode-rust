pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if (1..nums.len()).all(|i| nums[0] == nums[i]) {
            0
        } else {
            1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3674() {
        assert_eq!(Solution::min_operations(vec![1, 2]), 1);
        assert_eq!(Solution::min_operations(vec![5, 5, 5]), 0);
    }
}
