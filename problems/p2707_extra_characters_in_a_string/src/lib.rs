pub struct Solution {}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dp = vec![100; s.len() + 1];
        dp[0] = 0;

        for i in 0..s.len() {
            dp[i + 1] = dp[i] + 1;
            for j in 0..dictionary.len() {
                if i + 1 < dictionary[j].len() {
                    continue;
                }
                if &s[i + 1 - dictionary[j].len()..=i] == &dictionary[j] {
                    dp[i + 1] = dp[i + 1].min(dp[i + 1 - dictionary[j].len()]);
                }
            }
        }

        println!("dp: {:?}", dp);

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2707() {
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".to_string(),
                vec![
                    "leet".to_string(),
                    "code".to_string(),
                    "leetcode".to_string()
                ]
            ),
            1
        );
        assert_eq!(
            Solution::min_extra_char(
                "sayhelloworld".to_string(),
                vec!["hello".to_string(), "world".to_string()]
            ),
            3
        );
    }
}
