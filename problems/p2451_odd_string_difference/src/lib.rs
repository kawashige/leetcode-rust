pub struct Solution {}

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut word_with_differences = words
            .into_iter()
            .map(|w| {
                let differences = (0..w.len() - 1)
                    .map(|i| (w.as_bytes()[i + 1] as i32 - w.as_bytes()[i] as i32).to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                (differences, w)
            })
            .collect::<Vec<_>>();

        word_with_differences.sort_unstable();

        if word_with_differences[0].0 != word_with_differences[1].0 {
            word_with_differences[0].1.to_string()
        } else {
            word_with_differences[word_with_differences.len() - 1]
                .1
                .to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2451() {
        assert_eq!(
            Solution::odd_string(vec![
                "adc".to_string(),
                "wzy".to_string(),
                "abc".to_string()
            ]),
            "abc".to_string()
        );
        assert_eq!(
            Solution::odd_string(vec![
                "aaa".to_string(),
                "bob".to_string(),
                "ccc".to_string(),
                "ddd".to_string()
            ]),
            "bob".to_string()
        );
    }
}
