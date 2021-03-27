use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewels = jewels.chars().collect::<HashSet<char>>();
        stones.chars().filter(|c| jewels.contains(c)).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0771() {
        assert_eq!(
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
        assert_eq!(
            Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
            0
        );
    }
}
