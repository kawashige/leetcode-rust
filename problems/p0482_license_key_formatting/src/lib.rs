pub struct Solution {}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let ss = s.replace("-", "").to_ascii_uppercase();
        let first_group_count = if ss.len() % k as usize == 0 {
            k as usize
        } else {
            ss.len() % k as usize
        };
        (first_group_count..=ss.len())
            .step_by(k as usize)
            .map(|i| {
                (&ss[if i == first_group_count {
                    0
                } else {
                    i - k as usize
                }..i])
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("-")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0482() {
        assert_eq!(
            "5F3Z-2E9W".to_string(),
            Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
        );
        assert_eq!(
            "2-5G-3J".to_string(),
            Solution::license_key_formatting("2-5g-3-J".to_string(), 2)
        );
        assert_eq!(
            "2-53-JG-GG".to_string(),
            Solution::license_key_formatting("2-53J-G-G-G".to_string(), 2)
        );
    }
}
