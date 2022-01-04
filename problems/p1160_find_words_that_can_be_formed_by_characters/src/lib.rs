pub struct Solution {}

impl Solution {
    pub fn char_count(word: &str) -> [usize; 26] {
        word.chars().fold([0; 26], |mut count, c| {
            count[c as usize - 0x61] += 1;
            count
        })
    }

    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let count = Self::char_count(&chars);
        words
            .into_iter()
            .map(|word| {
                let word_count = Self::char_count(&word);
                if count.iter().zip(word_count.iter()).all(|(a, b)| a >= b) {
                    word.len() as i32
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
    fn test_1160() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    "cat".to_string().to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            ),
            6
        );
        assert_eq!(
            Solution::count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string()
                ],
                "welldonehoneyr".to_string()
            ),
            10
        );
    }
}
