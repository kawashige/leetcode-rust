pub struct Solution {}

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        (0..sequence.len())
            .map(|i| {
                (i..sequence.len())
                    .step_by(word.len())
                    .take_while(|j| sequence[*j..].starts_with(&word))
                    .count()
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1668() {
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ab".to_string()),
            2
        );
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ba".to_string()),
            1
        );
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ac".to_string()),
            0
        );
        assert_eq!(
            Solution::max_repeating(
                "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string(),
                "aaaba".to_string()
            ),
            5
        );
    }
}
