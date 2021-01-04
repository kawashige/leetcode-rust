pub struct Solution {}

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let l = nums.len();
        let mut dp = vec![vec![0; l + 1]; l + 1];

        for i in 1..=l {
            for j in 0..=(l - i) {
                if (l - i) % 2 == 0 {
                    dp[j][j + i] = std::cmp::max(
                        dp[j + 1][j + i] + nums[j],
                        dp[j][j + i - 1] + nums[j + i - 1],
                    );
                } else {
                    dp[j][j + i] = std::cmp::min(
                        dp[j + 1][j + i] - nums[j],
                        dp[j][j + i - 1] - nums[j + i - 1],
                    );
                }
            }
        }

        println!("dp: {:?}", dp);
        0 <= dp[0][l]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0486() {
        assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
        assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
        assert!(Solution::predict_the_winner(vec![1, 5, 2, 4, 6]));
    }
}
