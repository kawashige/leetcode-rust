pub struct Solution {}

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut left = vec![nums[0]; nums.len()];
        for i in 1..nums.len() {
            left[i] = left[i - 1].min(nums[i - 1]);
        }
        let mut right = vec![nums[nums.len() - 1]; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            right[i] = right[i + 1].min(nums[i + 1]);
        }
        (1..nums.len() - 1)
            .filter(|i| left[*i] < nums[*i] && nums[*i] > right[*i])
            .map(|i| left[i] + right[i] + nums[i])
            .min()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2909() {
        assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
