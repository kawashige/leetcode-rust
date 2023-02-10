pub struct Solution {}

impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut chars = n.chars().collect::<Vec<_>>();
        let x = (x as u8 + b'0') as char;
        let i = if n.starts_with('-') {
            (1..n.len()).find(|i| chars[*i] > x).unwrap_or(chars.len())
        } else {
            (0..n.len()).find(|i| chars[*i] < x).unwrap_or(chars.len())
        };

        chars.insert(i, x);
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1881() {
        assert_eq!(
            Solution::max_value("28824579515".to_string(), 8),
            "828824579515".to_string()
        );
        assert_eq!(Solution::max_value("99".to_string(), 9), "999".to_string());
        assert_eq!(
            Solution::max_value("-13".to_string(), 2),
            "-123".to_string()
        );
        assert_eq!(
            Solution::max_value("-132".to_string(), 3),
            "-1323".to_string()
        );
    }
}
