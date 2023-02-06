pub struct Solution {}

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        (0..nums.len() / 2)
            .map(|i| nums[i] + nums[nums.len() - 1 - i])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1877() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }
}
