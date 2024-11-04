pub struct Solution {}

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut count = 0;
        let mut result = String::new();

        for i in 0..word.len() {
            if (0 < i && word.as_bytes()[i - 1] != word.as_bytes()[i]) || count == 9 {
                result += &format!("{}{}", count, word.as_bytes()[i - 1] as char);
                count = 0;
            }
            count += 1;
        }
        result += &format!("{}{}", count, word.as_bytes()[word.len() - 1] as char);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3163() {
        assert_eq!(
            Solution::compressed_string("abcde".to_string()),
            "1a1b1c1d1e".to_string()
        );
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()),
            "9a5a2b".to_string()
        );
    }
}
