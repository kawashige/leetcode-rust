pub struct Solution {}

impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut dp = vec![vec![false; 4901]; mat.len() + 1];
        dp[0][0] = true;

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                for k in 0..dp[i].len() {
                    if dp[i][k] {
                        dp[i + 1][k + mat[i][j] as usize] = true;
                    }
                }
            }
        }

        (0..dp[0].len())
            .filter(|i| dp.last().unwrap()[*i])
            .map(|i| (i as i32 - target).abs())
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1981() {
        assert_eq!(
            Solution::minimize_the_difference(
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                13
            ),
            0
        );
        assert_eq!(
            Solution::minimize_the_difference(vec![vec![1], vec![2], vec![3]], 100),
            94
        );
        assert_eq!(
            Solution::minimize_the_difference(vec![vec![1, 2, 9, 8, 7]], 6),
            1
        );
    }
}
