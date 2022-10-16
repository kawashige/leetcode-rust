pub struct Solution {}

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let mut dp = vec![vec![std::i32::MAX; d + 1]; job_difficulty.len()];

        for i in 0..job_difficulty.len() {
            let mut day_difficulty = job_difficulty[i];
            for j in (0..i).rev() {
                for k in 1..d {
                    if dp[j][k] != std::i32::MAX {
                        dp[i][k + 1] = dp[i][k + 1].min(dp[j][k] + day_difficulty);
                    }
                }
                day_difficulty = day_difficulty.max(job_difficulty[j]);
            }
            dp[i][1] = day_difficulty;
        }

        if dp[job_difficulty.len() - 1][d] == std::i32::MAX {
            -1
        } else {
            dp[job_difficulty.len() - 1][d]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1335() {
        assert_eq!(Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
        assert_eq!(Solution::min_difficulty(vec![9, 9, 9], 4), -1);
        assert_eq!(Solution::min_difficulty(vec![1, 1, 1], 3), 3);
    }
}
