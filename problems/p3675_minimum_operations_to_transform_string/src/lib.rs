pub struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if let Some(b) = s.as_bytes().iter().filter(|b| b != &&b'a').min() {
            26 - (b - b'a') as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3675() {
        assert_eq!(Solution::min_operations("yz".to_string()), 2);
        assert_eq!(Solution::min_operations("a".to_string()), 0);
    }
}
