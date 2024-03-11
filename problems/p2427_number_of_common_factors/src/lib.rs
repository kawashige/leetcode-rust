pub struct Solution {}

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        (1..=a.min(b)).filter(|i| a % i == 0 && b % i == 0).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2427() {
        assert_eq!(Solution::common_factors(12, 6), 4);
        assert_eq!(Solution::common_factors(25, 30), 2);
    }
}
