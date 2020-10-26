pub struct Solution {}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0292() {
        assert!(Solution::can_win_nim(1));
        assert!(Solution::can_win_nim(2));
        assert!(!Solution::can_win_nim(4));
        assert!(Solution::can_win_nim(5));
        assert!(!Solution::can_win_nim(8));
    }
}
