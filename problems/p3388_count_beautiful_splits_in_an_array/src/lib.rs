pub struct Solution {}

impl Solution {
    pub fn beautiful_splits(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut dp = vec![vec![0; nums.len() + 1]; nums.len() + 1];
        for i in (0..nums.len()).rev() {
            for j in i..nums.len() {
                if nums[i] == nums[j] {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                }
            }
        }

        let mut result = 0;
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                if (i + 1 <= j - i && dp[0][i + 1] >= i + 1)
                    || (j - i < nums.len() - j && dp[i + 1][j + 1] >= j - i)
                {
                    result += 1;
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
    fn test_3388() {
        assert_eq!(Solution::beautiful_splits(vec![1, 1, 2, 1]), 2);
        assert_eq!(Solution::beautiful_splits(vec![1, 2, 3, 4]), 0);
    }
}
