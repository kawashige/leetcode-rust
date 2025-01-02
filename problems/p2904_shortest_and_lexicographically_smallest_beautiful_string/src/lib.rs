pub struct Solution {}

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let mut count = vec![0; s.len() + 1];
        for i in 0..s.len() {
            count[i + 1] += count[i];
            count[i + 1] += (s.as_bytes()[i] - b'0') as i32;
        }

        let mut strings = Vec::new();
        for i in 0..s.len() {
            for j in 0..=i {
                if count[i + 1] - count[j] == k {
                    strings.push(&s[j..=i]);
                }
            }
        }
        if strings.is_empty() {
            return String::new();
        }
        strings
            .into_iter()
            .min_by_key(|s| (s.len(), *s))
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2904() {
        assert_eq!(
            Solution::shortest_beautiful_substring("100011001".to_string(), 3),
            "11001".to_string()
        );
        assert_eq!(
            Solution::shortest_beautiful_substring("1011".to_string(), 2),
            "11".to_string()
        );
    }
}
