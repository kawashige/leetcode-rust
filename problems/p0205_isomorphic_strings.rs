pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        if s.len() != t.len() {
            return false;
        }

        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();
        for (s_c, t_c) in s.chars().zip(t.chars()) {
            match (s_map.get(&s_c), t_map.get(&t_c)) {
                (Some(s), Some(t)) => {
                    if &s_c != t || &t_c != s {
                        return false;
                    }
                }
                (None, None) => {
                    s_map.insert(s_c, t_c);
                    t_map.insert(t_c, s_c);
                }
                (_, _) => {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0205() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "bar".to_string(),
            "foo".to_string()
        ));
    }
}
