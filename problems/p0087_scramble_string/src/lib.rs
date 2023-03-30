use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count(s: &str) -> [usize; 26] {
        s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        })
    }

    pub fn check(s1: &str, s2: &str, memo: &mut HashMap<(String, String), bool>) -> bool {
        if let Some(result) = memo.get(&(s1.to_string(), s2.to_string())) {
            return *result;
        }
        let result = if s1.len() == 1 {
            s1 == s2
        } else if Self::count(s1) != Self::count(s2) {
            false
        } else {
            for i in 1..s1.len() {
                if (Self::check(&s1[..i], &s2[..i], memo) && Self::check(&s1[i..], &s2[i..], memo))
                    || (Self::check(&s1[..i], &s2[s2.len() - i..], memo)
                        && Self::check(&s1[i..], &s2[..s2.len() - i], memo))
                {
                    return true;
                }
            }
            false
        };
        memo.insert((s1.to_string(), s2.to_string()), result);
        result
    }

    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::check(&s1, &s2, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0087() {
        assert!(Solution::is_scramble(
            "eebaacbcbcadaaedceaaacadccd".to_string(),
            "eadcaacabaddaceacbceaabeccd".to_string()
        ));
        assert!(Solution::is_scramble(
            "great".to_string(),
            "rgeat".to_string()
        ));
        assert!(!Solution::is_scramble(
            "abcde".to_string(),
            "caebd".to_string()
        ));
        assert!(Solution::is_scramble("a".to_string(), "a".to_string()));
    }
}
