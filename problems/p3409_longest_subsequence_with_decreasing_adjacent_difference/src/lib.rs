pub struct Solution {}

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 300]; nums.len()];
        let mut indices = vec![-1; 301];
        let mut result = 1;

        for i in 0..nums.len() {
            let mut tmp = 1;
            for d in (0..300).rev() {
                if nums[i] + d <= 300 {
                    let j = (nums[i] + d) as usize;
                    if indices[j] != -1 {
                        tmp = tmp.max(dp[indices[j] as usize][d as usize] + 1)
                    }
                }
                if 1 + d <= nums[i] {
                    let j = (nums[i] - d) as usize;
                    if indices[j] != -1 {
                        tmp = tmp.max(dp[indices[j] as usize][d as usize] + 1)
                    }
                }
                dp[i][d as usize] = tmp;
            }
            indices[nums[i] as usize] = i as i32;
            result = result.max(tmp);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3409() {
        assert_eq!(Solution::longest_subsequence(vec![68, 83, 52, 70, 52]), 4);
        assert_eq!(Solution::longest_subsequence(vec![4, 5, 6]), 3);
        assert_eq!(Solution::longest_subsequence(vec![16, 6, 3]), 3);
        assert_eq!(Solution::longest_subsequence(vec![6, 5, 3, 4, 2, 1]), 4);
        assert_eq!(
            Solution::longest_subsequence(vec![10, 20, 10, 19, 10, 20]),
            5
        );
    }
}
