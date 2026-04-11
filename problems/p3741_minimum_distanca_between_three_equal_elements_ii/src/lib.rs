pub struct Solution {}

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut nums = nums.into_iter().zip(0..).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut result = std::i32::MAX;
        for i in 0..nums.len() {
            if 1 < i && nums[i].0 == nums[i - 2].0 {
                result = result.min((nums[i].1 - nums[i - 2].1) * 2);
            }
        }

        if result == std::i32::MAX { -1 } else { result }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3741() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
        assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
        assert_eq!(Solution::minimum_distance(vec![1]), -1);
    }
}
