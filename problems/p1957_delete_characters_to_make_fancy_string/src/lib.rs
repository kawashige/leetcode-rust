pub struct Solution {}

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut count = 0;

        for i in 0..s.len() {
            if 0 < i && s.as_bytes()[i - 1] == s.as_bytes()[i] {
                count += 1;
            } else {
                count = 1;
            }
            if count < 3 {
                result.push(s.as_bytes()[i] as char);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1957() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::make_fancy_string("aaabaaaa".to_string()),
            "aabaa".to_string()
        );
        assert_eq!(
            Solution::make_fancy_string("aab".to_string()),
            "aab".to_string()
        );
    }
}
