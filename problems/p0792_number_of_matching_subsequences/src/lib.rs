pub struct Solution {}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let indices = s
            .chars()
            .enumerate()
            .fold(vec![Vec::new(); 26], |mut v, (i, c)| {
                v[c as usize - 0x61].push(i + 1);
                v
            });

        words
            .into_iter()
            .filter(|w| {
                let mut prev = 0;
                for c in w.chars() {
                    let v = &indices[c as usize - 0x61];
                    let i = match v.binary_search(&prev) {
                        Ok(i) => i + 1,
                        Err(i) => i,
                    };

                    if i < v.len() {
                        prev = v[i];
                    } else {
                        return false;
                    }
                }
                true
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0792() {
        assert_eq!(
            Solution::num_matching_subseq(
                "abcde".to_string(),
                vec![
                    "a".to_string(),
                    "bb".to_string(),
                    "acd".to_string(),
                    "ace".to_string()
                ]
            ),
            3
        );

        assert_eq!(
            Solution::num_matching_subseq(
                "dsahjpjauf".to_string(),
                vec![
                    "ahjpjau".to_string(),
                    "ja".to_string(),
                    "ahbwzgqnuk".to_string(),
                    "tnmlanowax".to_string()
                ]
            ),
            2
        );
    }
}
