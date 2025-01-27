pub struct Solution {}

impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut max = 0;

        for i in 0..nums.len() {
            for j in 0..=i {
                if (nums[i] - nums[j]).abs() <= nums[i].min(nums[j]) {
                    max = max.max(nums[i] ^ nums[j]);
                }
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2932() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![10, 100]), 0);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![5, 6, 25, 30]), 7);
    }
}
