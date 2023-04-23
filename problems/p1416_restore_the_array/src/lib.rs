pub struct Solution {}

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let k = k as usize;

        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;

        for i in 0..s.len() {
            let mut num = 0;
            let mut pow = 1;
            for j in (0..=i).rev() {
                num += (s.as_bytes()[j] - b'0') as usize * pow;
                if k < num || k < pow {
                    break;
                }
                pow *= 10;
                if 0 < num && s.as_bytes()[j] != b'0' {
                    dp[i + 1] += dp[j];
                    dp[i + 1] %= M;
                }
            }
        }

        *dp.last().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1416() {
        assert_eq!(Solution::number_of_arrays("2020".to_string(), 30), 1);
        assert_eq!(Solution::number_of_arrays("1000".to_string(), 10000), 1);
        assert_eq!(Solution::number_of_arrays("1000".to_string(), 10), 0);
        assert_eq!(Solution::number_of_arrays("1317".to_string(), 2000), 8);
    }
}
