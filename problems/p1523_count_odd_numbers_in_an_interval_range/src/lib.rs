pub struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high + 1) / 2 - low / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1523() {
        assert_eq!(Solution::count_odds(3, 7), 3);
        assert_eq!(Solution::count_odds(8, 10), 1);
    }
}
