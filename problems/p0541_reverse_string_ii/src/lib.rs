pub struct Solution {}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        s.chars()
            .collect::<Vec<char>>()
            .chunks(k as usize)
            .enumerate()
            .map(|(i, chunk)| {
                if i % 2 == 0 {
                    chunk.iter().rev().collect::<String>()
                } else {
                    chunk.iter().collect::<String>()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0541() {
        assert_eq!(
            "bacdfeg".to_string(),
            Solution::reverse_str("abcdefg".to_string(), 2)
        );
        assert_eq!(
            "abcdefg".to_string(),
            Solution::reverse_str("abcdefg".to_string(), 1)
        );
        assert_eq!("a".to_string(), Solution::reverse_str("a".to_string(), 2));
    }
}
