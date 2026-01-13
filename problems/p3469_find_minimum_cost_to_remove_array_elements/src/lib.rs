pub struct Solution {}

impl Solution {
    pub fn min_cost(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.into_iter().max().unwrap();
        }
        let n = nums.len() + 1;

        let mut dp = vec![vec![std::i32::MAX; n]; n];
        dp[0][0] = 0;

        for i in 0..nums.len() {
            for j in 0..=i {
                if dp[i][j] == std::i32::MAX {
                    continue;
                }
                if i == j {
                    if i + 2 == n {
                        dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j] + nums[i]);
                    }
                    if i + 2 < n {
                        // i, i + 1
                        dp[i + 2][i + 2] =
                            dp[i + 2][i + 2].min(dp[i][j] + nums[i].max(nums[i + 1]));
                    }
                    if i + 3 < n {
                        // i, i + 2
                        dp[i + 3][i + 1] =
                            dp[i + 3][i + 1].min(dp[i][j] + nums[i].max(nums[i + 2]));
                        // i + 1, i + 2
                        dp[i + 3][i] = dp[i + 3][i].min(dp[i][j] + nums[i + 1].max(nums[i + 2]));
                    }
                } else {
                    if i + 1 < n {
                        // i, j
                        dp[i + 1][i + 1] = dp[i + 1][i + 1].min(dp[i][j] + nums[i].max(nums[j]));
                    }
                    if i + 2 < n {
                        // i + 1, j
                        dp[i + 2][i] = dp[i + 2][i].min(dp[i][j] + nums[i + 1].max(nums[j]));
                        // i, i + 1
                        dp[i + 2][j] = dp[i + 2][j].min(dp[i][j] + nums[i].max(nums[i + 1]));
                    }
                }
            }
        }

        println!("{:?}", dp);

        dp[n - 1]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if x == &std::i32::MAX {
                    None
                } else {
                    Some(x + if i == n - 1 { 0 } else { nums[i] })
                }
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3469() {
        assert_eq!(Solution::min_cost(vec![22, 15, 7]), 29);
        assert_eq!(Solution::min_cost(vec![9, 1, 5]), 10);
        assert_eq!(Solution::min_cost(vec![6, 2, 8, 4]), 12);
        assert_eq!(Solution::min_cost(vec![2, 1, 3, 3]), 5);
    }
}
