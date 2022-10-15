pub struct Solution {}

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let k = k as usize;
        if s.len() == k {
            return 0;
        }

        let bytes = s.as_bytes();
        let mut dp = vec![vec![vec![1000; s.len() + 2]; k + 1]; s.len()];

        for i in 0..s.len() {
            // j個消す
            for j in 0..=k {
                if i < j + 1 {
                    // iより前が全部消せる
                    dp[i][k - i][1] = 1;
                    break;
                }

                // i - j - 1番目と比較
                for l in j..=k {
                    for m in 1..=s.len() {
                        if bytes[i] == bytes[i - j - 1] {
                            dp[i][l - j][m + 1] = dp[i][l - j][m + 1].min(
                                dp[i - j - 1][l][m] + if [1, 9, 99].contains(&m) { 1 } else { 0 },
                            );
                        } else {
                            dp[i][l - j][1] = dp[i][l - j][1].min(dp[i - j - 1][l][m] + 1);
                        }
                    }
                }
            }
        }

        let mut result = std::i32::MAX;

        for i in 0..=k {
            // s.len() - 1 - i番目のi個以上消せるものをチェック
            for j in 0..=k - i {
                for l in 0..=s.len() {
                    result = result.min(dp[s.len() - 1 - i][k - j][l]);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0000() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aa".to_string(), 1),
            1
        );
        assert_eq!(
            Solution::get_length_of_optimal_compression("a".to_string(), 1),
            0
        );
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaabcccd".to_string(), 2),
            4
        );
        assert_eq!(
            Solution::get_length_of_optimal_compression("aabbaa".to_string(), 2),
            2
        );
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0),
            3
        );
    }
}
