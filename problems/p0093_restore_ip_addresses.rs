pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn is_invalid(s: &str) -> bool {
            3 < s.len() || (s.starts_with('0') && 1 < s.len()) || 255 < s.parse::<i32>().unwrap()
        }

        let mut results = Vec::new();
        for i in 1..std::cmp::min(4, s.len()) {
            if is_invalid(&s[..i]) {
                continue;
            }
            for j in (i + 1)..std::cmp::min(i + 4, s.len()) {
                if is_invalid(&s[i..j]) {
                    continue;
                }
                for k in (j + 1)..std::cmp::min(j + 4, s.len()) {
                    if is_invalid(&s[j..k]) || is_invalid(&s[k..]) {
                        continue;
                    }
                    results.push(format!("{}.{}.{}.{}", &s[..i], &s[i..j], &s[j..k], &s[k..]));
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0093() {
        assert_eq!(
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()],
            Solution::restore_ip_addresses("25525511135".to_string())
        );
        assert_eq!(
            vec!["0.0.0.0".to_string()],
            Solution::restore_ip_addresses("0000".to_string())
        );
        assert_eq!(
            vec!["1.1.1.1".to_string()],
            Solution::restore_ip_addresses("1111".to_string())
        );
        assert_eq!(
            vec!["0.10.0.10".to_string(), "0.100.1.0".to_string()],
            Solution::restore_ip_addresses("010010".to_string())
        );
        assert_eq!(
            vec![
                "1.0.10.23".to_string(),
                "1.0.102.3".to_string(),
                "10.1.0.23".to_string(),
                "10.10.2.3".to_string(),
                "101.0.2.3".to_string()
            ],
            Solution::restore_ip_addresses("101023".to_string())
        );
    }
}
