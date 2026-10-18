pub struct Solution {}

impl Solution {
    pub fn min_removals(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![vec![-1; 20_001]; nums.len() + 1];
        dp[0][0] = 0;

        for i in 0..nums.len() {
            dp[i + 1][0] = dp[i][0].max(0);
            for j in 0..dp[i].len() {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                if dp[i][j] != -1 && (j ^ nums[i] as usize) < dp[i].len() {
                    dp[i + 1][j ^ nums[i] as usize] =
                        dp[i + 1][j ^ nums[i] as usize].max(dp[i][j] + 1);
                }
            }
        }

        if dp.last().unwrap()[target as usize] == -1 {
            -1
        } else {
            nums.len() as i32 - dp.last().unwrap()[target as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3877() {
        assert_eq!(Solution::min_removals(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::min_removals(vec![2, 4], 1), -1);
        assert_eq!(Solution::min_removals(vec![7], 7), 0);
    }
}

fn main() {}
