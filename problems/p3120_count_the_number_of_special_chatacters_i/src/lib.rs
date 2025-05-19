pub struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut count = [[false; 26]; 2];
        for i in 0..word.len() {
            if word.as_bytes()[i].is_ascii_lowercase() {
                count[0][(word.as_bytes()[i] - b'a') as usize] = true;
            } else {
                count[1][(word.as_bytes()[i] - b'A') as usize] = true;
            }
        }
        (0..26).filter(|i| count[0][*i] && count[1][*i]).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3120() {
        assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
        assert_eq!(Solution::number_of_special_chars("abBCab".to_string()), 1);
    }
}
