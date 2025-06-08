pub struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut upper = vec![false; 26];
        let mut lower = vec![false; 26];
        let mut lower_after_upper = vec![false; 26];

        for i in 0..word.len() {
            if word.as_bytes()[i].is_ascii_lowercase() {
                lower[(word.as_bytes()[i] - b'a') as usize] = true;
                if upper[(word.as_bytes()[i] - b'a') as usize] {
                    lower_after_upper[(word.as_bytes()[i] - b'a') as usize] = true
                }
            } else {
                upper[(word.as_bytes()[i] - b'A') as usize] = true;
            }
        }

        (0..26)
            .filter(|i| upper[*i] && lower[*i] && !lower_after_upper[*i])
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3121() {
        assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
        assert_eq!(Solution::number_of_special_chars("AbBCab".to_string()), 0);
    }
}
