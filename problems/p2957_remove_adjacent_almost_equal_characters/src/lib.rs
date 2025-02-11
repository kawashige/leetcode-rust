pub struct Solution {}

impl Solution {
    pub fn remove_almost_equal_characters(word: String) -> i32 {
        let mut result = 0;

        let mut i = 1;
        while i < word.len() {
            if (word.as_bytes()[i] as i32 - word.as_bytes()[i - 1] as i32).abs() < 2 {
                result += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2957() {
        assert_eq!(
            Solution::remove_almost_equal_characters("aaaaa".to_string()),
            2
        );
        assert_eq!(
            Solution::remove_almost_equal_characters("abddez".to_string()),
            2
        );
        assert_eq!(
            Solution::remove_almost_equal_characters("zyxyxyz".to_string()),
            3
        );
    }
}
