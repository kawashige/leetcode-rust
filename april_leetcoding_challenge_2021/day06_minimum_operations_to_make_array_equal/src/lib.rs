pub struct Solution {}

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        n * n / 4
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day06() {
        assert_eq!(Solution::min_operations(3), 2);
        assert_eq!(Solution::min_operations(6), 9);
    }
}
