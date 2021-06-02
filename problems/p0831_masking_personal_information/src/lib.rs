pub struct Solution {}

impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains("@") {
            let s = s.to_ascii_lowercase();
            let mut sp = s.split("@");
            let mut first = sp.next().unwrap().chars();

            format!(
                "{}*****{}@{}",
                first.next().unwrap(),
                first.next_back().unwrap(),
                sp.next().unwrap()
            )
        } else {
            let digits = s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
            let country = if digits.len() < 11 {
                "".to_string()
            } else {
                format!(
                    "+{}-",
                    std::iter::repeat('*')
                        .take(digits.len() - 10)
                        .collect::<String>()
                )
            };

            format!(
                "{}***-***-{}",
                country,
                digits[(digits.len() - 4)..].iter().collect::<String>()
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0831() {
        assert_eq!(
            Solution::mask_pii("LeetCode@LeetCode.com".to_string()),
            "l*****e@leetcode.com".to_string()
        );
        assert_eq!(
            Solution::mask_pii("AB@qq.com".to_string()),
            "a*****b@qq.com".to_string()
        );
        assert_eq!(
            Solution::mask_pii("1(234)567-890".to_string()),
            "***-***-7890".to_string()
        );
        assert_eq!(
            Solution::mask_pii("86-(10)12345678".to_string()),
            "+**-***-***-5678".to_string()
        );
    }
}
