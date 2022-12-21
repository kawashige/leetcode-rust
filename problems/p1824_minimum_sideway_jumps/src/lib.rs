pub struct Solution {}

impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let mut dp = vec![vec![std::i32::MAX; 4]; obstacles.len()];
        dp[0][2] = 0;
        for i in 1..obstacles.len() {
            for lane in 1..4 {
                if obstacles[i] == lane || obstacles[i - 1] == lane {
                    continue;
                }
                for prev_lane in 1..4 {
                    if obstacles[i - 1] == prev_lane
                        || dp[i - 1][prev_lane as usize] == std::i32::MAX
                    {
                        continue;
                    }
                    dp[i][lane as usize] = dp[i][lane as usize]
                        .min(dp[i - 1][prev_lane as usize] + if lane == prev_lane { 0 } else { 1 });
                }
            }
        }
        *dp.iter().last().unwrap().iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1824() {
        assert_eq!(Solution::min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
        assert_eq!(Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
        assert_eq!(Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
    }
}
