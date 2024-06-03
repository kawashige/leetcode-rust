pub struct Solution {}

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        (n - 1).max(1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2549() {
        assert_eq!(Solution::distinct_integers(5), 4);
        assert_eq!(Solution::distinct_integers(3), 2);
    }
}
