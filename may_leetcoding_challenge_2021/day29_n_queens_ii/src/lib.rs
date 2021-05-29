pub struct Solution {}

impl Solution {
    pub fn dfs(
        board: &mut Vec<Vec<char>>,
        i: usize,
        row: usize,
        diagonal1: usize,
        diagonal2: usize,
        result: &mut i32,
    ) {
        if i == board[0].len() {
            *result += 1;
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

    pub fn total_n_queens(n: i32) -> i32 {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut result = 0;

        Self::dfs(&mut board, 0, 0, 0, 0, &mut result);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day22() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
