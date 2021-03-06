pub struct Solution {}

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

        let mut max = 0;
        for i in 0..a.len() {
            for j in 0..b.len() {
                if a[i] == b[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                    max = std::cmp::max(max, dp[i + 1][j + 1]);
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_718() {
        assert_eq!(
            Solution::find_length(vec![0, 1, 1, 1, 1], vec![1, 0, 1, 0, 1]),
            2
        );
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
            3
        );
    }
}
