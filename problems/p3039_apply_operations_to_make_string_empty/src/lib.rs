pub struct Solution {}

impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut count = [0; 26];
        let mut max_count = 0;
        let mut result = String::new();

        for i in 0..s.len() {
            count[(s.as_bytes()[i] - b'a') as usize] += 1;
            if max_count < count[(s.as_bytes()[i] - b'a') as usize] {
                max_count = count[(s.as_bytes()[i] - b'a') as usize];
                result.clear();
                result.push(s.as_bytes()[i] as char);
            } else if max_count == count[(s.as_bytes()[i] - b'a') as usize] {
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
    fn test_3039() {
        assert_eq!(
            Solution::last_non_empty_string("aabcbbca".to_string()),
            "ba".to_string()
        );
        assert_eq!(
            Solution::last_non_empty_string("abcd".to_string()),
            "abcd".to_string()
        );
    }
}
