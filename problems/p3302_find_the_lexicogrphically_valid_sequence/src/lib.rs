pub struct Solution {}

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let mut dp = vec![0; word1.len()];
        if word1.as_bytes()[word1.len() - 1] == word2.as_bytes()[word2.len() - 1] {
            dp[word1.len() - 1] = 1;
        }

        for i in (0..word1.len() - 1).rev() {
            dp[i] = dp[i + 1]
                + if dp[i + 1] < word2.len()
                    && word1.as_bytes()[i] == word2.as_bytes()[word2.len() - 1 - dp[i + 1]]
                {
                    1
                } else {
                    0
                };
        }
        let mut j = 0;
        let mut result = Vec::new();
        let mut is_changed = false;
        for i in 0..dp.len() {
            if word1.as_bytes()[i] == word2.as_bytes()[j] {
                result.push(i as i32);
                j += 1;
            } else if !is_changed {
                if word2.len() <= result.len() + if i + 1 < dp.len() { dp[i + 1] } else { 0 } + 1 {
                    is_changed = true;
                    result.push(i as i32);
                    j += 1;
                }
            }
            if result.len() == word2.len() {
                break;
            }
        }
        if result.len() == word2.len() {
            result
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3302() {
        assert_eq!(
            Solution::valid_sequence("vbcca".to_string(), "abc".to_string()),
            vec![0, 1, 2]
        );
        assert_eq!(
            Solution::valid_sequence("bacdc".to_string(), "abc".to_string()),
            vec![1, 2, 4]
        );
        assert_eq!(
            Solution::valid_sequence("aaaaaa".to_string(), "aaabc".to_string()),
            vec![]
        );
    }
}
