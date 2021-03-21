pub struct Solution {}

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut board = vec![vec![1; n]; n];

        for mine in mines {
            board[mine[0] as usize][mine[1] as usize] = 0;
        }

        let mut left = vec![vec![0; n]; n];
        let mut right = vec![vec![0; n]; n];
        let mut up = vec![vec![0; n]; n];
        let mut down = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 1..n {
                left[i][j] = if board[i][j - 1] == 0 {
                    0
                } else {
                    left[i][j - 1] + 1
                };
                right[i][n - 1 - j] = if board[i][n - j] == 0 {
                    0
                } else {
                    right[i][n - j] + 1
                };
                up[j][i] = if board[j - 1][i] == 0 {
                    0
                } else {
                    up[j - 1][i] + 1
                };
                down[n - 1 - j][i] = if board[n - j][i] == 0 {
                    0
                } else {
                    down[n - j][i] + 1
                };
            }
        }

        let mut max = 0;
        for i in 0..n {
            for j in 0..n {
                if board[i][j] == 1 {
                    max = std::cmp::max(
                        max,
                        left[i][j].min(right[i][j]).min(up[i][j]).min(down[i][j]) + 1,
                    );
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
    fn test_0764() {
        assert_eq!(
            Solution::order_of_largest_plus_sign(2, vec![vec![0, 1], vec![1, 0], vec![1, 1]]),
            1
        );
        assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
        assert_eq!(Solution::order_of_largest_plus_sign(2, vec![]), 1);
        assert_eq!(Solution::order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0);
    }
}
