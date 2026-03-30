pub struct Solution {}

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut codes = (0..code.len())
            .filter_map(|i| {
                if !code[i].is_empty()
                    && code[i].chars().all(|c| c.is_alphanumeric() || c == '_')
                    && ["electronics", "grocery", "pharmacy", "restaurant"]
                        .contains(&business_line[i].as_str())
                    && is_active[i]
                {
                    Some((business_line[i].to_string(), code[i].to_string()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        codes.sort_unstable();
        codes.into_iter().map(|(_, code)| code).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3606() {
        assert_eq!(
            Solution::validate_coupons(
                vec![
                    "SAVE20".to_string(),
                    "".to_string(),
                    "PHARMA5".to_string(),
                    "SAVE@20".to_string()
                ],
                vec![
                    "restaurant".to_string(),
                    "grocery".to_string(),
                    "pharmacy".to_string(),
                    "restaurant".to_string()
                ],
                vec![true, true, true, true]
            ),
            vec!["PHARMA5".to_string(), "SAVE20".to_string()]
        );
        assert_eq!(
            Solution::validate_coupons(
                vec![
                    "GROCERY15".to_string(),
                    "ELECTRONICS_50".to_string(),
                    "DISCOUNT10".to_string()
                ],
                vec![
                    "grocery".to_string(),
                    "electronics".to_string(),
                    "invalid".to_string()
                ],
                vec![false, true, true]
            ),
            vec!["ELECTRONICS_50".to_string()]
        );
    }
}
