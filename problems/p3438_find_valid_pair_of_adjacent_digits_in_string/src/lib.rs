pub struct Solution {}

impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let mut count = [0; 26];
        for b in s.as_bytes() {
            count[(b - b'0') as usize] += 1;
        }
        for i in 0..s.len() - 1 {
            if s.as_bytes()[i] != s.as_bytes()[i + 1]
                && s.as_bytes()[i] - b'0' == count[(s.as_bytes()[i] - b'0') as usize]
                && s.as_bytes()[i + 1] - b'0' == count[(s.as_bytes()[i + 1] - b'0') as usize]
            {
                return s[i..i + 2].to_string();
            }
        }

        Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3438() {
        assert_eq!(
            Solution::find_valid_pair("2523533".to_string()),
            "23".to_string()
        );
        assert_eq!(
            Solution::find_valid_pair("221".to_string()),
            "21".to_string()
        );
        assert_eq!(Solution::find_valid_pair("22".to_string()), "".to_string());
    }
}
