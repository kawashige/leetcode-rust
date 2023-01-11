pub struct Solution {}

impl Solution {
    pub fn check(prev: u128, s: &str) -> bool {
        if s.is_empty() {
            return true;
        }
        for i in 0..s.len() {
            let value = s[..=i].parse::<u128>().unwrap();
            if value + 1 == prev {
                if Self::check(value, &s[i + 1..]) {
                    return true;
                }
            }
            if prev <= value {
                return false;
            }
        }
        false
    }

    pub fn split_string(s: String) -> bool {
        for i in 0..s.len() - 1 {
            let start = s[..=i].parse::<u128>().unwrap();
            if Self::check(start, &s[i + 1..]) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1849() {
        assert!(!Solution::split_string("1234".to_string()));
        assert!(Solution::split_string("050043".to_string()));
        assert!(!Solution::split_string("9080701".to_string()));
        assert!(!Solution::split_string("12345678901234567890".to_string()));
    }
}
