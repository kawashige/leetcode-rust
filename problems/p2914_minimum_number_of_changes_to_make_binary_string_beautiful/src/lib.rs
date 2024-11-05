pub struct Solution {}

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        (0..s.len())
            .step_by(2)
            .filter(|i| s.as_bytes()[*i] != s.as_bytes()[i + 1])
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2914() {
        assert_eq!(Solution::min_changes("1001".to_string()), 2);
        assert_eq!(Solution::min_changes("10".to_string()), 1);
        assert_eq!(Solution::min_changes("0000".to_string()), 0);
    }
}
