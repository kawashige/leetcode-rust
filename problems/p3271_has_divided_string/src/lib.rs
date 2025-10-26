pub struct Solution {}

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut result = String::new();
        for i in (0..s.len()).step_by(k as usize) {
            result.push(
                (((0..k as usize)
                    .map(|j| (s.as_bytes()[i + j] - b'a') as usize)
                    .sum::<usize>()
                    % 26) as u8
                    + b'a') as char,
            );
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3271() {
        assert_eq!(
            Solution::string_hash("abcd".to_string(), 2),
            "bf".to_string()
        );
        assert_eq!(Solution::string_hash("mxz".to_string(), 3), "i".to_string());
    }
}
