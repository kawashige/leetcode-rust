pub struct Solution {}

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let chars = s
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<_>>();
        (1..chars.len())
            .filter(|i| chars[*i] != chars[i - 1])
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3019() {
        assert_eq!(Solution::count_key_changes("aAbBcC".to_string()), 2);
        assert_eq!(Solution::count_key_changes("AaAaAaaA".to_string()), 0);
    }
}
