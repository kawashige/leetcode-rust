use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(char::is_alphabetic)
            .filter(|x| !x.is_empty())
            .map(|x| x.trim_start_matches("0"))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1805() {
        assert_eq!(
            Solution::num_different_integers("a123bc34d8ef34".to_string()),
            3
        );
        assert_eq!(
            Solution::num_different_integers("leet1234code234".to_string()),
            2
        );
        assert_eq!(Solution::num_different_integers("a1b01c001".to_string()), 1);
    }
}
