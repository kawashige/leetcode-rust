pub struct Solution {}

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut result = 0;
        let mut occupied = 0;
        for i in 0..s.len() {
            if s.as_bytes()[i] == b'E' {
                occupied += 1;
                result = result.max(occupied);
            } else {
                occupied -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3168() {
        assert_eq!(Solution::minimum_chairs("EEEEEEE".to_string()), 7);
        assert_eq!(Solution::minimum_chairs("ELELEEL".to_string()), 2);
        assert_eq!(Solution::minimum_chairs("ELEELEELLL".to_string()), 3);
    }
}
