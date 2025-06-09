pub struct Solution {}

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut indices = vec![-1; 26];
        for i in 0..s.len() {
            indices[(s.as_bytes()[i] - b'a') as usize] = i as i32;
        }
        let mut result = 0;
        for i in 0..t.len() {
            result += (i as i32 - indices[(t.as_bytes()[i] - b'a') as usize]).abs();
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3146() {
        assert_eq!(
            Solution::find_permutation_difference("abc".to_string(), "bac".to_string()),
            2
        );
        assert_eq!(
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()),
            12
        );
    }
}
