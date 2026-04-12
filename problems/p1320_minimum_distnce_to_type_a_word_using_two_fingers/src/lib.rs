pub struct Solution {}

impl Solution {
    pub fn dist(i: usize, j: usize, char_pos: &Vec<(i32, i32)>) -> i32 {
        (char_pos[i].0 - char_pos[j].0).abs() + (char_pos[i].1 - char_pos[j].1).abs()
    }

    pub fn minimum_distance(word: String) -> i32 {
        let char_pos = ('A'..='Z')
            .map(|c| {
                let i = (c as u8 - b'A') as i32;
                (i / 6, i % 6)
            })
            .collect::<Vec<_>>();

        let mut dp = vec![vec![vec![std::i32::MAX; 26]; 26]; word.len() + 1];
        for i in 0..26 {
            for j in 0..26 {
                dp[0][i][j] = 0;
            }
        }

        for k in 0..word.len() {
            let current_char = (word.as_bytes()[k] - b'A') as usize;
            for i in 0..26 {
                for j in 0..26 {
                    if dp[k][i][j] != std::i32::MAX {
                        dp[k + 1][current_char][j] = dp[k + 1][current_char][j]
                            .min(dp[k][i][j] + Self::dist(i, current_char, &char_pos));
                        dp[k + 1][i][current_char] = dp[k + 1][i][current_char]
                            .min(dp[k][i][j] + Self::dist(j, current_char, &char_pos));
                    }
                }
            }
        }

        let mut result = std::i32::MAX;
        for i in 0..26 {
            for j in 0..26 {
                result = result.min(dp[word.len()][i][j]);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1320() {
        assert_eq!(Solution::minimum_distance("CAKE".to_string()), 3);
        assert_eq!(Solution::minimum_distance("HAPPY".to_string()), 6);
    }
}
