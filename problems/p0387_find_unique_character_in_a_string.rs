pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = HashMap::new();
        let chars = s.chars().collect::<Vec<_>>();
        for c in &chars {
            *counts.entry(c).or_insert(0) += 1;
        }
        for i in 0..chars.len() {
            if counts[&chars[i]] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0387() {
        assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()));
        assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
        assert_eq!(-1, Solution::first_uniq_char("ss".to_string()));
    }
}
