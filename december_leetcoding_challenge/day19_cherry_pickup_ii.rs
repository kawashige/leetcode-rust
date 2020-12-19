pub struct Solution {}

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![vec![-1; grid[0].len()]; grid[0].len()]; grid.len()];

        let mut max = 0;
        dp[0][0][grid[0].len() - 1] = grid[0][0] + grid[0][grid[0].len() - 1];
        for i in 1..grid.len() {
            for j in 0..grid[0].len() {
                for k in 0..grid[0].len() {
                    let v = if j == k {
                        grid[i][j]
                    } else {
                        grid[i][j] + grid[i][k]
                    };
                    let col1_min = if j == 0 { 0 } else { j - 1 };
                    let col1_max = if j == grid[0].len() - 1 { j } else { j + 1 };
                    let col2_min = if k == 0 { 0 } else { k - 1 };
                    let col2_max = if k == grid[0].len() - 1 { k } else { k + 1 };
                    for c1 in col1_min..=col1_max {
                        for c2 in col2_min..=col2_max {
                            if dp[i - 1][c1][c2] >= 0 {
                                dp[i][j][k] = std::cmp::max(dp[i][j][k], v + dp[i - 1][c1][c2]);
                            }
                        }
                    }
                    if i == grid.len() - 1 {
                        max = std::cmp::max(max, dp[i][j][k]);
                    }
                }
            }
        }
        println!("dp: {:?}", dp);
        max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day19() {
        assert_eq!(
            28,
            Solution::cherry_pickup(vec![
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 0, 3, 0],
                vec![2, 0, 9, 0, 0, 0, 0],
                vec![0, 3, 0, 5, 4, 0, 0],
                vec![1, 0, 2, 3, 0, 0, 6]
            ])
        );
        assert_eq!(
            22,
            Solution::cherry_pickup(vec![
                vec![1, 0, 0, 3],
                vec![0, 0, 0, 3],
                vec![0, 0, 3, 3],
                vec![9, 0, 3, 3]
            ])
        );
        assert_eq!(4, Solution::cherry_pickup(vec![vec![1, 1], vec![1, 1]]));
        assert_eq!(
            96,
            Solution::cherry_pickup(vec![
                vec![0, 8, 7, 10, 9, 10, 0, 9, 6],
                vec![8, 7, 10, 8, 7, 4, 9, 6, 10],
                vec![8, 1, 1, 5, 1, 5, 5, 1, 2],
                vec![9, 4, 10, 8, 8, 1, 9, 5, 0],
                vec![4, 3, 6, 10, 9, 2, 4, 8, 10],
                vec![7, 3, 2, 8, 3, 3, 5, 9, 8],
                vec![1, 2, 6, 5, 6, 2, 0, 10, 0]
            ])
        );
        assert_eq!(
            96,
            Solution::cherry_pickup(vec![
                vec![0, 0, 10, 2, 8, 4, 0],
                vec![7, 9, 3, 5, 4, 8, 3],
                vec![6, 9, 8, 3, 5, 6, 0],
                vec![0, 4, 1, 1, 9, 3, 7],
                vec![5, 6, 9, 8, 8, 10, 10],
                vec![9, 2, 9, 7, 4, 8, 3],
                vec![1, 6, 1, 2, 0, 9, 9]
            ])
        );
    }
}
