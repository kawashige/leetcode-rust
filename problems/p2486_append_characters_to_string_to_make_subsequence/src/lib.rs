pub struct Solution {}

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut i = 0;
        for j in 0..s.len() {
            if s.as_bytes()[j] == t.as_bytes()[i] {
                i += 1;
                if i == t.len() {
                    break;
                }
            }
        }
        (t.len() - i) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2486() {
        assert_eq!(
            Solution::append_characters("coaching".to_string(), "coading".to_string()),
            4
        );
        assert_eq!(
            Solution::append_characters("abcde".to_string(), "a".to_string()),
            0
        );
        assert_eq!(
            Solution::append_characters("z".to_string(), "abcde".to_string()),
            5
        );
    }
}
