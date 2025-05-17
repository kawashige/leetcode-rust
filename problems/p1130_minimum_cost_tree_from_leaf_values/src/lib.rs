pub struct Solution {}

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut max = vec![vec![0; arr.len()]; arr.len()];
        for i in 0..arr.len() {
            let mut num = 0;
            for j in i..arr.len() {
                num = num.max(arr[j]);
                max[i][j] = num;
            }
        }

        let mut dp = vec![vec![std::i32::MAX; arr.len()]; arr.len()];
        for i in 0..arr.len() {
            dp[i][i] = 0;
        }

        for l in 1..arr.len() {
            for i in 0..arr.len() - l {
                for k in i..i + l {
                    dp[i][i + l] = dp[i][i + l]
                        .min(max[i][k] * max[k + 1][i + l] + dp[i][k] + dp[k + 1][i + l]);
                }
            }
        }

        dp[0][arr.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1130() {
        assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
        assert_eq!(Solution::mct_from_leaf_values(vec![4, 11]), 44);
    }
}
