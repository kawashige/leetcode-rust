pub struct Solution {}

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 5 {
            return 0;
        }

        nums.sort_unstable();

        (0..4)
            .map(|i| nums[nums.len() - 4 + i] - nums[i])
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1509() {
        assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
        assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
    }
}
