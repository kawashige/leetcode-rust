pub struct Solution {}

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut r = (0..s.len())
            .rev()
            .find(|i| s.as_bytes()[*i] == b'0')
            .unwrap_or(s.len());
        if r == s.len() {
            return 0;
        }

        let mut result = 0;
        for i in (0..r).rev() {
            if s.as_bytes()[i] == b'0' {
                continue;
            }
            result += (r - i) as i64;
            r -= 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2938() {
        assert_eq!(Solution::minimum_steps("101".to_string()), 1);
        assert_eq!(Solution::minimum_steps("100".to_string()), 2);
        assert_eq!(Solution::minimum_steps("0111".to_string()), 0);
    }
}
