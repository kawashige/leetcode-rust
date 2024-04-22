pub struct Solution {}

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let chars = words
            .into_iter()
            .map(|w| {
                w.as_bytes()
                    .iter()
                    .fold(0, |acc, b| acc | 1 << (b - b'a') as usize)
            })
            .collect::<Vec<_>>();
        let mut count = 0;

        for i in 0..chars.len() {
            for j in i + 1..chars.len() {
                if chars[i] == chars[j] {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2506() {
        assert_eq!(
            Solution::similar_pairs(vec![
                "aba".to_string(),
                "aabb".to_string(),
                "abcd".to_string(),
                "bac".to_string(),
                "aabc".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::similar_pairs(vec!["aabb".to_string(), "ab".to_string(), "ba".to_string()]),
            3
        );
        assert_eq!(
            Solution::similar_pairs(vec![
                "nba".to_string(),
                "cba".to_string(),
                "dba".to_string()
            ]),
            0
        );
    }
}
