pub struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let different = (0..s1.len())
            .filter(|i| s1.as_bytes()[*i] != s2.as_bytes()[*i])
            .collect::<Vec<_>>();
        different.is_empty()
            || (different.len() == 2
                && (s1.as_bytes()[different[0]] == s2.as_bytes()[different[1]]
                    && s1.as_bytes()[different[1]] == s2.as_bytes()[different[0]]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1790() {
        assert!(Solution::are_almost_equal(
            "bank".to_string(),
            "kanb".to_string()
        ));
        assert!(!Solution::are_almost_equal(
            "attack".to_string(),
            "defend".to_string()
        ));
        assert!(Solution::are_almost_equal(
            "kelb".to_string(),
            "kelb".to_string()
        ));
    }
}
