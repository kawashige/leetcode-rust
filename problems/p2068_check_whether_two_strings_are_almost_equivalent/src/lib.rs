pub struct Solution {}

impl Solution {
    pub fn count(s: &str) -> [i32; 26] {
        s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        })
    }

    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        Self::count(&word1)
            .into_iter()
            .zip(Self::count(&word2).into_iter())
            .all(|(c1, c2)| (c1 - c2).abs() < 4)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2068() {
        assert!(!Solution::check_almost_equivalent(
            "aaaa".to_string(),
            "bccb".to_string()
        ));
        assert!(Solution::check_almost_equivalent(
            "abcdeef".to_string(),
            "abaaacc".to_string()
        ));
        assert!(Solution::check_almost_equivalent(
            "cccddabba".to_string(),
            "babababab".to_string()
        ));
    }
}
