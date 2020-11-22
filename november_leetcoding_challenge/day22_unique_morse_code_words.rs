pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse_codes = vec![
            ".-".to_string(),
            "-...".to_string(),
            "-.-.".to_string(),
            "-..".to_string(),
            ".".to_string(),
            "..-.".to_string(),
            "--.".to_string(),
            "....".to_string(),
            "..".to_string(),
            ".---".to_string(),
            "-.-".to_string(),
            ".-..".to_string(),
            "--".to_string(),
            "-.".to_string(),
            "---".to_string(),
            ".--.".to_string(),
            "--.-".to_string(),
            ".-.".to_string(),
            "...".to_string(),
            "-".to_string(),
            "..-".to_string(),
            "...-".to_string(),
            ".--".to_string(),
            "-..-".to_string(),
            "-.--".to_string(),
            "--..".to_string(),
        ];

        words
            .into_iter()
            .map(|w| {
                w.chars()
                    .map(|c| morse_codes[c as usize - 0x61].to_string())
                    .collect::<String>()
            })
            .collect::<HashSet<String>>()
            .len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day22() {
        assert_eq!(
            2,
            Solution::unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ])
        );
    }
}
