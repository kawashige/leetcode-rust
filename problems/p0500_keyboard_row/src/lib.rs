pub struct Solution {}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut row_map = [0; 26];
        ["qwertyuiop", "asdfghjkl", "zxcvbnm"]
            .iter()
            .enumerate()
            .for_each(|(i, s)| {
                s.chars().for_each(|c| {
                    row_map[c as usize - 0x61] = i;
                });
            });

        words
            .into_iter()
            .filter(|w| {
                if w.is_empty() {
                    return true;
                }
                let lower_w = w.to_ascii_lowercase();
                let mut chars = lower_w.chars();
                let row_no = row_map[chars.next().unwrap() as usize - 0x61];
                chars.all(|c| row_map[c as usize - 0x61] == row_no)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0500() {
        assert_eq!(
            vec!["Alaska".to_string(), "Dad".to_string()],
            Solution::find_words(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ])
        );
        assert_eq!(
            vec!["".to_string()],
            Solution::find_words(vec!["".to_string(),])
        )
    }
}
