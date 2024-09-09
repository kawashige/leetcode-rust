pub struct Solution {}

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut result = 0;
        let mut seen = [false; 26];
        for i in 0..s.len() {
            if seen[(s.as_bytes()[i] - b'a') as usize] {
                continue;
            }
            seen[(s.as_bytes()[i] - b'a') as usize] = true;
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2716() {
        assert_eq!(Solution::minimized_string_length("aaabc".to_string()), 3);
        assert_eq!(Solution::minimized_string_length("cbbd".to_string()), 3);
        assert_eq!(Solution::minimized_string_length("baadccab".to_string()), 4);
    }
}
