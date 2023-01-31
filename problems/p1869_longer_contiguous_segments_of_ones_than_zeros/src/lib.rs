pub struct Solution {}

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut len = [0; 2];
        let mut count = 1;

        for i in 1..s.len() {
            if s.as_bytes()[i - 1] != s.as_bytes()[i] {
                len[(s.as_bytes()[i - 1] - b'0') as usize] =
                    len[(s.as_bytes()[i - 1] - b'0') as usize].max(count);
                count = 0;
            }
            count += 1;
        }
        len[(s.as_bytes()[s.len() - 1] - b'0') as usize] =
            len[(s.as_bytes()[s.len() - 1] - b'0') as usize].max(count);

        len[0] < len[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1869() {
        assert!(Solution::check_zero_ones("1101".to_string()));
        assert!(!Solution::check_zero_ones("111000".to_string()));
        assert!(!Solution::check_zero_ones("110100010".to_string()));
    }
}
