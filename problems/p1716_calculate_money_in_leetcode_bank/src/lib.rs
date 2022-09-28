pub struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        (1..=n).map(|d| (d - 1) / 7 + 1 + (d - 1) % 7).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1716() {
        assert_eq!(Solution::total_money(4), 10);
        assert_eq!(Solution::total_money(10), 37);
        assert_eq!(Solution::total_money(20), 96);
    }
}
