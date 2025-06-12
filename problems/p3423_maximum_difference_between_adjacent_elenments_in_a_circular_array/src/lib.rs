pub struct Solution {}

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .map(|i| (nums[i] - nums[(i + 1) % nums.len()]).abs())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3423() {
        assert_eq!(Solution::max_adjacent_distance(vec![1, 2, 4]), 3);
        assert_eq!(Solution::max_adjacent_distance(vec![-5, -10, -5]), 5);
    }
}
