pub struct Solution {}

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut concat = String::new();
        for word in words {
            concat += &word;
            if s == concat {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1961() {
        assert!(Solution::is_prefix_string(
            "iloveleetcode".to_string(),
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                "apples".to_string()
            ]
        ));
        assert!(!Solution::is_prefix_string(
            "iloveleetcode".to_string(),
            vec![
                "apples".to_string(),
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string()
            ]
        ));
    }
}
