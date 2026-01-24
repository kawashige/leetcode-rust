pub struct Solution {}

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => 1,
            2 => 2,
            _ => 1 << (32 - (nums.len() as i32).leading_zeros()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3513() {
        assert_eq!(Solution::unique_xor_triplets(vec![1, 2]), 2);
        assert_eq!(Solution::unique_xor_triplets(vec![3, 1, 2]), 4);
    }
}
