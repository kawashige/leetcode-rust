pub struct Solution {}

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let mut count = vec![[0; 26]; s.len() + 1];
        for i in 0..s.len() {
            if 0 < i {
                count[i + 1] = count[i].clone();
            }
            count[i + 1][(s.as_bytes()[i] - b'a') as usize] += 1;
        }
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 0;

        for i in 0..s.len() {
            dp[i + 1] = dp[i] + 1;
            for j in 0..i {
                let mut tmp = -1;
                let mut is_ok = true;
                for k in 0..26 {
                    if count[i + 1][k] - count[j][k] == 0 {
                        continue;
                    } else if tmp != -1 && count[i + 1][k] - count[j][k] != tmp {
                        is_ok = false;
                        break;
                    }
                    tmp = count[i + 1][k] - count[j][k];
                }
                if is_ok {
                    dp[i + 1] = dp[i + 1].min(dp[j] + 1);
                }
            }
        }

        dp[dp.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3144() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("fabccddg".to_string()),
            3
        );
        assert_eq!(
            Solution::minimum_substrings_in_partition("abababaccddb".to_string()),
            2
        );
    }
}
