pub struct Solution {}

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1025() {
        assert!(Solution::divisor_game(2));
        assert!(!Solution::divisor_game(3));
    }
}
