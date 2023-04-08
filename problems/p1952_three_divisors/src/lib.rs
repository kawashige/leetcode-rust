pub struct Solution {}

impl Solution {
    pub fn is_three(n: i32) -> bool {
        (1..=n).filter(|i| n % i == 0).count() == 3
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1952() {
        assert!(!Solution::is_three(2));
        assert!(Solution::is_three(4));
    }
}
