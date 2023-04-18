pub struct Solution {}

impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![1_000_000 * nums.len(); k as usize + 2]; nums.len() + 1];
        dp[0][0] = 0;

        for i in 0..nums.len() {
            let mut sum = 0;
            let mut max = 0;
            for j in (0..=i).rev() {
                sum += nums[j] as usize;
                max = max.max(nums[j] as usize);

                for l in 0..k as usize + 1 {
                    dp[i + 1][l + 1] =
                        dp[i + 1][l + 1].min(dp[j][l] + (max * (i - j + 1) - sum) as usize);
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
    fn test_1959() {
        assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20], 0), 10);
        assert_eq!(
            Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1),
            10
        );
        assert_eq!(
            Solution::min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2),
            15
        );
    }
}
