pub struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result = String::new();
        let mut counts: Vec<usize> = Vec::new();
        let mut strings = Vec::new();
        let mut repeat_count = 0;
        for c in s.chars() {
            if c.is_numeric() {
                repeat_count = repeat_count * 10 + c.to_digit(10).unwrap() as usize;
            } else if c == '[' {
                strings.push(String::new());
                counts.push(repeat_count);
                repeat_count = 0;
            } else if c == ']' {
                let repeated_string = strings.pop().unwrap().repeat(counts.pop().unwrap());
                if strings.is_empty() {
                    result.push_str(&mut &repeated_string);
                } else {
                    strings.last_mut().unwrap().push_str(&mut &repeated_string);
                }
            } else if strings.is_empty() {
                result.push(c);
            } else {
                strings.last_mut().unwrap().push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day19() {
        assert_eq!(
            "aaabcbc".to_string(),
            Solution::decode_string("3[a]2[bc]".to_string())
        );
        assert_eq!(
            "accaccacc".to_string(),
            Solution::decode_string("3[a2[c]]".to_string())
        );
        assert_eq!(
            "abcabccdcdcdef".to_string(),
            Solution::decode_string("2[abc]3[cd]ef".to_string())
        );
        assert_eq!(
            "abccdcdcdxyz".to_string(),
            Solution::decode_string("abc3[cd]xyz".to_string())
        );
    }
}
