pub struct Solution {}

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        n * (n + 1) / 2 - m * (n / m) * (n / m + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2894() {
        assert_eq!(Solution::difference_of_sums(10, 3), 19);
        assert_eq!(Solution::difference_of_sums(5, 6), 15);
        assert_eq!(Solution::difference_of_sums(5, 1), -15);
    }
}
