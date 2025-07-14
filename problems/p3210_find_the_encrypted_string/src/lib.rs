pub struct Solution {}

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        (0..s.len())
            .map(|i| s.as_bytes()[(i + k as usize) % s.len()] as char)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3210() {
        assert_eq!(
            Solution::get_encrypted_string("dart".to_string(), 3),
            "tdar".to_string()
        );
        assert_eq!(
            Solution::get_encrypted_string("aaa".to_string(), 1),
            "aaa".to_string()
        );
    }
}
