pub struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut dp = vec![vec![std::i32::MIN; nums.len()]; nums.len()];
        dp[0][nums.len() - 1] = 0;

        let mut result = std::i32::MIN;
        for i in 0..multipliers.len() {
            for j in 0..=i {
                let left = j;
                let right = nums.len() - 1 - (i - j);

                if left + 1 < nums.len() {
                    dp[left + 1][right] =
                        dp[left + 1][right].max(dp[left][right] + multipliers[i] * nums[left]);
                    if i == multipliers.len() - 1 {
                        result = result.max(dp[left + 1][right]);
                    }
                }
                if 0 < right {
                    dp[left][right - 1] =
                        dp[left][right - 1].max(dp[left][right] + multipliers[i] * nums[right]);
                    if i == multipliers.len() - 1 {
                        result = result.max(dp[left][right - 1]);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1770() {
        assert_eq!(
            Solution::maximum_score(
                vec![
                    -854, -941, 10, 299, 995, -346, 294, -393, 351, -76, 210, 897, -651, 920, 624,
                    969, -629, 985, -695, 236, 637, -901, -817, 546, -69, 192, -377, 251, 542,
                    -316, -879, -764, -560, 927, 629, 877, 42, 381, 367, -549, 602, 139, -312,
                    -281, 105, 690, -376, -705, -906, 85, -608, 639, 752, 770, -139, -601, 341, 61,
                    969, 276, 176, -715, -545, 471, -170, -126, 596, -737, 130
                ],
                vec![
                    83, 315, -442, -714, 461, 920, -737, -93, -818, -760, 558, -584, -358, -228,
                    -220
                ]
            ),
            3040819
        );
        assert_eq!(Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]), 14);
        assert_eq!(
            Solution::maximum_score(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6]),
            102
        );
    }
}
