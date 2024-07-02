pub struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        words[left as usize..=right as usize]
            .iter()
            .filter(|w| w.starts_with(&VOWELS) && w.ends_with(&VOWELS))
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2586() {
        assert_eq!(
            Solution::vowel_strings(
                vec!["are".to_string(), "amy".to_string(), "u".to_string()],
                0,
                2
            ),
            2
        );
        assert_eq!(
            Solution::vowel_strings(
                vec![
                    "hey".to_string(),
                    "aeo".to_string(),
                    "mu".to_string(),
                    "ooo".to_string(),
                    "artro".to_string()
                ],
                1,
                4
            ),
            3
        );
    }
}
