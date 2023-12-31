pub struct Solution {}

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, num| acc | num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2317() {
        assert_eq!(Solution::maximum_xor(vec![3, 2, 4, 6]), 7);
        assert_eq!(Solution::maximum_xor(vec![1, 2, 3, 9, 2]), 11);
    }
}
