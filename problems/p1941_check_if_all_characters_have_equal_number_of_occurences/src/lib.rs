use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        s.as_bytes()
            .iter()
            .fold([0; 26], |mut count, b| {
                count[(b - b'a') as usize] += 1;
                count
            })
            .iter()
            .filter(|c| &&0 < c)
            .collect::<HashSet<_>>()
            .len()
            == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1941() {
        assert!(Solution::are_occurrences_equal("abacbc".to_string()));
        assert!(!Solution::are_occurrences_equal("aaabb".to_string()));
    }
}
