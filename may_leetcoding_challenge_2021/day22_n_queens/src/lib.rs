pub struct Solution {}

impl Solution {
    pub fn dfs(
        board: &mut Vec<Vec<char>>,
        i: usize,
        row: usize,
        diagonal1: usize,
        diagonal2: usize,
        result: &mut Vec<Vec<String>>,
    ) {
        if i == board[0].len() {
            result.push(
                board
                    .iter()
                    .map(|v| v.into_iter().collect::<String>())
                    .collect(),
            );
            return;
        }

        for j in 0..board.len() {
            if row & 1 << j > 0
                || diagonal1 & 1 << (i + j) > 0
                || diagonal2 & 1 << (board.len() + i - j) > 0
            {
                continue;
            }

            board[j][i] = 'Q';
            Self::dfs(
                board,
                i + 1,
                row | 1 << j,
                diagonal1 | 1 << (i + j),
                diagonal2 | 1 << (board.len() + i - j),
                result,
            );
            board[j][i] = '.';
        }
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut result = Vec::new();

        Self::dfs(&mut board, 0, 0, 0, 0, &mut result);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day22() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ],
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
            ]
        );

        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q".to_string()]]);
        assert_eq!(Solution::solve_n_queens(9), vec![vec!["Q".to_string()]]);
    }
}
