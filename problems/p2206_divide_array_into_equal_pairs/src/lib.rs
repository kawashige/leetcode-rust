pub struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        (0..nums.len() - 1)
            .step_by(2)
            .all(|i| nums[i + 1] == nums[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2206() {
        assert!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
        assert!(!Solution::divide_array(vec![1, 2, 3, 4]));
    }
}
