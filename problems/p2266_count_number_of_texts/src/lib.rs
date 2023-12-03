pub struct Solution {}

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        const M: usize = 1_000_000_007;

        let mut same_key = 1;

        let mut dp = vec![0; pressed_keys.len() + 1];
        dp[0] = 1;

        for i in 0..pressed_keys.len() {
            if 0 < i && pressed_keys.as_bytes()[i - 1] == pressed_keys.as_bytes()[i] {
                same_key += 1;
            } else {
                same_key = 1;
            }
            let letters = match pressed_keys.as_bytes()[i] {
                b'7' | b'9' => 4,
                _ => 3,
            };
            for j in 0..letters.min(same_key) {
                if i < j {
                    break;
                }
                dp[i + 1] += dp[i - j];
                dp[i + 1] %= M;
                if j == letters.min(same_key) {
                    dp[i + 1] += dp[i - j];
                    dp[i + 1] %= M;
                }
            }
        }
        println!("dp: {:?}", dp);
        *dp.last().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2266() {
        assert_eq!(Solution::count_texts("22233".to_string()), 8);
        assert_eq!(
            Solution::count_texts("222222222222222222222222222222222222".to_string()),
            82876089
        );
    }
}
