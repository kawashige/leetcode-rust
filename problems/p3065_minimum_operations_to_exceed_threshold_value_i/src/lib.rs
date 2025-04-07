pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().filter(|n| n < &k).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3065() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 3);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 1), 0);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 9), 4);
    }
}
