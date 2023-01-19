pub struct Solution {}

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut s = s.split_ascii_whitespace().collect::<Vec<_>>();
        s.sort_unstable_by_key(|s| s.as_bytes()[s.len() - 1]);
        let s = s
            .into_iter()
            .map(|s| s.trim_end_matches(char::is_numeric))
            .collect::<Vec<_>>();
        s.join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1859() {
        assert_eq!(
            Solution::sort_sentence("is2 sentence4 This1 a3".to_string()),
            "This is a sentence".to_string()
        );
        assert_eq!(
            Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()),
            "Me Myself and I".to_string()
        );
    }
}
