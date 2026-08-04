pub struct Solution {}

impl Solution {
    pub fn largest_even(s: String) -> String {
        if let Some(i) = (0..s.len()).rev().find(|i| s.as_bytes()[*i] == b'2') {
            s[..=i].to_string()
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3798() {
        assert_eq!(
            Solution::largest_even("1112".to_string()),
            "1112".to_string()
        );
        assert_eq!(Solution::largest_even("221".to_string()), "22".to_string());
        assert_eq!(Solution::largest_even("1".to_string()), "".to_string());
    }
}

fn main() {}
