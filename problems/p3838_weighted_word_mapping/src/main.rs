pub struct Solution {}

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .into_iter()
            .map(|word| {
                ((25 - word
                    .as_bytes()
                    .iter()
                    .map(|b| weights[(b - b'a') as usize])
                    .sum::<i32>()
                    % 26) as u8
                    + b'a') as char
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3838() {
        assert_eq!(
            Solution::map_word_weights(
                vec!["abcd".to_string(), "def".to_string(), "xyz".to_string()],
                vec![
                    5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7,
                    2
                ]
            ),
            "rij".to_string()
        );
        assert_eq!(
            Solution::map_word_weights(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                ]
            ),
            "yyy".to_string()
        );
        assert_eq!(
            Solution::map_word_weights(
                vec!["abcd".to_string()],
                vec![
                    7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5
                ]
            ),
            "g".to_string()
        );
    }
}

fn main() {}
