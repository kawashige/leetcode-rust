pub struct Solution {}

impl Solution {
    pub fn recurse(n: i32, d: i32) -> i32 {
        if n / d == 0 {
            return 0;
        }
        (n / d) / 10 * d
            + match (n / d) % 10 {
                0 => 0,
                1 => n % d + 1,
                _ => d,
            }
            + Self::recurse(n, d * 10)
    }

    pub fn count_digit_one(n: i32) -> i32 {
        Self::recurse(n, 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0233() {
        assert_eq!(Solution::count_digit_one(100), 6);
        assert_eq!(Solution::count_digit_one(13), 6);
        assert_eq!(Solution::count_digit_one(0), 0);
    }
}
