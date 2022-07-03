pub struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut chars = vec!['a'; s.len()];

        for (i, b) in s.as_bytes().iter().enumerate() {
            chars[indices[i] as usize] = *b as char;
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1528() {
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
    }
}
