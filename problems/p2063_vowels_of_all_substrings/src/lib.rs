pub struct Solution {}

impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        (0..word.len())
            .map(|i| {
                if [b'a', b'e', b'i', b'o', b'u'].contains(&word.as_bytes()[i]) {
                    (i as i64 + 1) * (word.len() as i64 - i as i64)
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2063() {
        assert_eq!(Solution::count_vowels("aba".to_string()), 6);
        assert_eq!(Solution::count_vowels("abc".to_string()), 3);
        assert_eq!(Solution::count_vowels("ltcd".to_string()), 0);
    }
}
