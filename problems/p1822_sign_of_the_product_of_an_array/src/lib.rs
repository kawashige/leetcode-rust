pub struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(1, |acc, num| acc * num.signum())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1822() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
