pub struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut max = 0;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                dp[i + 1][j + 1] = if nums1[i] == nums2[j] {
                    1 + dp[i][j]
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                };
                max = max.max(dp[i + 1][j + 1]);
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1035() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]),
            2
        );
        assert_eq!(
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
            3
        );
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
            2
        );
    }
}
