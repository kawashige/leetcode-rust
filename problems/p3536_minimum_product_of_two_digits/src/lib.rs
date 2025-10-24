pub struct Solution {}

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut digits = Vec::new();
        let mut n = n;
        while 0 < n {
            digits.push(n % 10);
            n /= 10;
        }
        digits.sort_unstable_by_key(|d| -d);
        digits[0] * digits[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3536() {
        assert_eq!(Solution::max_product(31), 3);
        assert_eq!(Solution::max_product(22), 4);
        assert_eq!(Solution::max_product(124), 8);
    }
}
