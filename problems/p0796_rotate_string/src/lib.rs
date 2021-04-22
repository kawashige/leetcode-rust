pub struct Solution {}

impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        if a.is_empty() && b.is_empty() {
            return true;
        }

        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        (0..a_bytes.len())
            .any(|i| (0..a_bytes.len()).all(|j| b_bytes[j] == a_bytes[(i + j) % a_bytes.len()]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0796() {
        assert!(Solution::rotate_string(
            "abcde".to_string(),
            "cdeab".to_string()
        ));
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "abced".to_string()
        ));
    }
}
