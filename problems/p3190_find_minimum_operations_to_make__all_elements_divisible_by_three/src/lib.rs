pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|x| match x % 3 {
                0 => 0,
                _ => 1,
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3190() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
    }
}
