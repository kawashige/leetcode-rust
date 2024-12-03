pub struct Solution {}

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1 = s1
            .as_bytes()
            .iter()
            .zip(0..)
            .map(|(b, i)| (i % 2, b))
            .collect::<Vec<_>>();
        let mut s2 = s2
            .as_bytes()
            .iter()
            .zip(0..)
            .map(|(b, i)| (i % 2, b))
            .collect::<Vec<_>>();
        s1.sort_unstable();
        s2.sort_unstable();
        (0..s1.len()).all(|i| s1[i].1 == s2[i].1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2839() {
        assert!(Solution::can_be_equal(
            "abcd".to_string(),
            "cdab".to_string()
        ));
        assert!(!Solution::can_be_equal(
            "abcd".to_string(),
            "dacb".to_string()
        ));
    }
}
