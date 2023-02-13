pub struct Solution {}

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        (1..=n).find(|i| n <= i * (i + 1) / 2).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1884() {
        assert_eq!(Solution::two_egg_drop(2), 2);
        assert_eq!(Solution::two_egg_drop(100), 14);
    }
}
