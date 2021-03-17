pub struct Solution {}

impl Solution {
    pub fn counts(w: &String) -> [usize; 26] {
        w.to_ascii_lowercase()
            .chars()
            .fold([0; 26], |mut counts, c| {
                if c.is_ascii_lowercase() {
                    counts[c as usize - 0x61] += 1;
                }
                counts
            })
    }

    pub fn shortest_completing_word(license_plate: String, mut words: Vec<String>) -> String {
        let counts = Self::counts(&license_plate);

        words.sort_by_key(|w| w.len());
        for w in words {
            let c = Self::counts(&w);
            if (0..26).all(|i| counts[i] <= c[i]) {
                return w;
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0748() {
        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 PSt".to_string(),
                vec![
                    "step".to_string(),
                    "steps".to_string(),
                    "stripe".to_string(),
                    "stepple".to_string()
                ]
            ),
            "steps".to_string()
        );

        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 456".to_string(),
                vec![
                    "looks".to_string(),
                    "pest".to_string(),
                    "stew".to_string(),
                    "show".to_string()
                ]
            ),
            "pest".to_string()
        );

        assert_eq!(
            Solution::shortest_completing_word(
                "Ah71752".to_string(),
                vec![
                    "suggest".to_string(),
                    "letter".to_string(),
                    "of".to_string(),
                    "husband".to_string(),
                    "easy".to_string(),
                    "education".to_string(),
                    "drug".to_string(),
                    "prevent".to_string(),
                    "writer".to_string(),
                    "old".to_string()
                ]
            ),
            "husband".to_string()
        );

        assert_eq!(
            Solution::shortest_completing_word(
                "OgEu755".to_string(),
                vec![
                    "enough".to_string(),
                    "these".to_string(),
                    "play".to_string(),
                    "wide".to_string(),
                    "wonder".to_string(),
                    "box".to_string(),
                    "arrive".to_string(),
                    "money".to_string(),
                    "tax".to_string(),
                    "thus".to_string()
                ]
            ),
            "enough".to_string()
        );

        assert_eq!(
            Solution::shortest_completing_word(
                "iMSlpe4".to_string(),
                vec![
                    "claim".to_string(),
                    "consumer".to_string(),
                    "student".to_string(),
                    "camera".to_string(),
                    "public".to_string(),
                    "never".to_string(),
                    "wonder".to_string(),
                    "simple".to_string(),
                    "thought".to_string(),
                    "use".to_string()
                ]
            ),
            "simple".to_string()
        );
    }
}
