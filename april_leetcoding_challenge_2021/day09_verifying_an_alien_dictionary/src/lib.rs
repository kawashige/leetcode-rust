pub struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let indices = order
            .as_bytes()
            .iter()
            .enumerate()
            .fold([0; 26], |mut indices, (i, b)| {
                indices[*b as usize - 0x61] = i as u8;
                indices
            });

        let words_indices = words
            .into_iter()
            .map(|w| {
                w.as_bytes()
                    .iter()
                    .map(|b| indices[*b as usize - 0x61])
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        words_indices.windows(2).all(|b| b[0] <= b[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day09() {
        assert!(Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ));
        assert!(!Solution::is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        ));
        assert!(!Solution::is_alien_sorted(
            vec!["apple".to_string(), "app".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));
    }
}
