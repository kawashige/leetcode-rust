pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().fold(k, |acc, num| acc ^ num).count_ones() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2997() {
        assert_eq!(Solution::min_operations(vec![2, 1, 3, 4], 1), 2);
        assert_eq!(Solution::min_operations(vec![2, 0, 2, 0], 0), 0);
    }
}
