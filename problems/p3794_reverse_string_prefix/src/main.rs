pub struct Solution {}

impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let k = k as usize;
        s[..k].chars().rev().chain(s[k..].chars()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3794() {
        assert_eq!(
            Solution::reverse_prefix("abcd".to_string(), 2),
            "bacd".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("xyz".to_string(), 3),
            "zyx".to_string()
        );
        assert_eq!(
            Solution::reverse_prefix("hey".to_string(), 1),
            "hey".to_string()
        );
    }
}

fn main() {}
