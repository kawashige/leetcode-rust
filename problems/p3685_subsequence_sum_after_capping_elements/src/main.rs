pub struct Solution {}

impl Solution {
    pub fn subsequence_sum_after_capping(nums: Vec<i32>, k: i32) -> Vec<bool> {
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable_by_key(|num| -num);

        let mut dp = vec![vec![false; k as usize + 1]; n + 1];
        dp[n][0] = true;
        for i in (0..nums.len()).rev() {
            dp[i][0] = true;
            for j in 0..dp[i].len() {
                if dp[i + 1][j] {
                    dp[i][j] = true;
                    if (j + nums[i] as usize) < dp[i].len() {
                        dp[i][j + nums[i] as usize] = true;
                    }
                }
            }
        }

        let mut result = vec![false; nums.len()];
        let mut j = n as i32 - 1;
        let mut t = n;

        for x in 1..=nums.len() as i32 {
            while 0 <= j && nums[j as usize] <= x {
                t -= 1;
                j -= 1;
            }
            for s in 0..dp[t].len() {
                if dp[t][s] && (k - s as i32) % x == 0 && (k - s as i32) / x <= t as i32 {
                    result[x as usize - 1] = true
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
    fn test_3685() {
        assert_eq!(
            Solution::subsequence_sum_after_capping(vec![1, 1], 3),
            vec![false, false]
        );
        assert_eq!(
            Solution::subsequence_sum_after_capping(vec![4, 3, 2, 4], 5),
            vec![false, false, true, true]
        );
        assert_eq!(
            Solution::subsequence_sum_after_capping(vec![1, 2, 3, 4, 5], 3),
            vec![true, true, true, true, true]
        );
    }
}
fn main() {}
