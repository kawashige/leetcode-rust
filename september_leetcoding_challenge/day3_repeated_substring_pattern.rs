pub struct Solution {}
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        for i in (1..chars.len()).filter(|n| chars.len() % n == 0) {
            if (1..(chars.len() / i)).all(|j| chars[..i] == chars[(i * j)..(i * j + i)]) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day3() {
        assert!(!Solution::repeated_substring_pattern("".to_string()));
        assert!(!Solution::repeated_substring_pattern("a".to_string()));
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_string()
        ));
    }
}
