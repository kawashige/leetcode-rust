pub struct Solution {}

impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, num| acc ^ num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2527() {
        assert_eq!(Solution::xor_beauty(vec![1, 4]), 5);
        assert_eq!(
            Solution::xor_beauty(vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30]),
            34
        );
    }
}
