pub struct Solution {}

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![std::i32::MIN; nums2.len()]; nums1.len()];

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if 0 < i {
                    dp[i][j] = dp[i - 1][j];
                }
                if 0 < j {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1]);
                }
                dp[i][j] = dp[i][j].max(
                    nums1[i] * nums2[j]
                        + if 0 < i && 0 < j {
                            dp[i - 1][j - 1].max(0)
                        } else {
                            0
                        },
                );
            }
        }

        dp[nums1.len() - 1][nums2.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1458() {
        assert_eq!(
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
            18
        );
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
        assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
    }
}
