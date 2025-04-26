pub struct Solution {}

impl Solution {
    pub fn min_operations(k: i32) -> i32 {
        (0..k).map(|i| i + (k + i) / (i + 1) - 1).min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3091() {
        assert_eq!(Solution::min_operations(11), 5);
        assert_eq!(Solution::min_operations(1), 0);
    }
}
