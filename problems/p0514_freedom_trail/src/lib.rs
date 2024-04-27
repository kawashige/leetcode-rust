pub struct Solution {}

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut dp = vec![vec![1_000_000; ring.len()]; key.len() + 1];
        dp[0][0] = 0;

        for i in 0..key.len() {
            for j in 0..ring.len() {
                if ring.as_bytes()[j] == key.as_bytes()[i] {
                    for k in 0..ring.len() {
                        dp[i + 1][j] = dp[i + 1][j].min(
                            dp[i][k]
                                + (j.max(k) - j.min(k)).min(j.min(k) + ring.len() - j.max(k))
                                + 1,
                        );
                    }
                }
            }
        }

        *dp.last().unwrap().into_iter().min().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0514() {
        assert_eq!(
            Solution::find_rotate_steps("abcde".to_string(), "ade".to_string()),
            6
        );
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
            4
        );
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "godding".to_string()),
            13
        );
    }
}
