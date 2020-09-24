pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        use std::collections::HashMap;
        let mut dic = HashMap::new();
        for c in s.chars() {
            let entry = dic.entry(c).or_insert(0);
            *entry += 1;
        }
        for c in t.chars() {
            let entry = dic.entry(c).or_insert(0);
            *entry -= 1;
        }
        dic.into_iter().find(|(_, v)| v < &0).unwrap().0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day23() {
        assert_eq!(
            'e',
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string())
        );
    }
}
