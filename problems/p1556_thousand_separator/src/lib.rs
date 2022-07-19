pub struct Solution {}

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        n.to_string()
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                if 0 < i && i % 3 == 0 {
                    vec!['.', c]
                } else {
                    vec![c]
                }
            })
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1556() {
        assert_eq!(Solution::thousand_separator(987), "987".to_string());
        assert_eq!(Solution::thousand_separator(1234), "1.234".to_string());
    }
}
