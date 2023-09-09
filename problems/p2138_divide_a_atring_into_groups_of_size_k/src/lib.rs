pub struct Solution {}

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        (0..(s.len() + k as usize - 1) / k as usize)
            .map(|i| {
                (0..k as usize)
                    .map(|j| {
                        if i * k as usize + j < s.len() {
                            s.as_bytes()[i * k as usize + j] as char
                        } else {
                            fill
                        }
                    })
                    .collect::<String>()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2138() {
        assert_eq!(
            Solution::divide_string("abcdefghi".to_string(), 3, 'x'),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
        assert_eq!(
            Solution::divide_string("abcdefghij".to_string(), 3, 'x'),
            vec![
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jxx".to_string()
            ]
        );
    }
}
