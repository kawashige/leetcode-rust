pub struct Solution {}

impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut s = String::new();

        for i in 0..target.len() {
            s.push('a');
            result.push(s.clone());
            while s.as_bytes().iter().last().unwrap() != &target.as_bytes()[i] {
                let c = s.pop().unwrap();
                s.push((b'a' + (c as u8 - b'a' + 1) % 26) as char);
                result.push(s.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3324() {
        assert_eq!(
            Solution::string_sequence("abc".to_string()),
            vec![
                "a".to_string(),
                "aa".to_string(),
                "ab".to_string(),
                "aba".to_string(),
                "abb".to_string(),
                "abc".to_string()
            ]
        );
        assert_eq!(
            Solution::string_sequence("he".to_string()),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
                "g".to_string(),
                "h".to_string(),
                "ha".to_string(),
                "hb".to_string(),
                "hc".to_string(),
                "hd".to_string(),
                "he".to_string()
            ]
        );
    }
}
