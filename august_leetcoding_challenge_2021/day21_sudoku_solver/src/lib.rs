pub struct Solution {}

impl Solution {
    pub fn dfs(
        i: usize,
        j: usize,
        board: &mut Vec<Vec<char>>,
        seen_row: &mut Vec<Vec<bool>>,
        seen_column: &mut Vec<Vec<bool>>,
        seen_grid: &mut Vec<Vec<bool>>,
    ) -> bool {
        if i > 8 {
            return true;
        }

        let (next_i, next_j) = if j == 8 { (i + 1, 0) } else { (i, j + 1) };

        if board[i][j] != '.' {
            Self::dfs(next_i, next_j, board, seen_row, seen_column, seen_grid)
        } else {
            for d in 1..10 {
                if seen_row[i][d] || seen_column[j][d] || seen_grid[i / 3 * 3 + j / 3][d] {
                    continue;
                }

                board[i][j] = (d as u8 + b'0') as char;
                seen_row[i][d] = true;
                seen_column[j][d] = true;
                seen_grid[i / 3 * 3 + j / 3][d] = true;

                if Self::dfs(next_i, next_j, board, seen_row, seen_column, seen_grid) {
                    return true;
                }

                board[i][j] = '.';
                seen_row[i][d] = false;
                seen_column[j][d] = false;
                seen_grid[i / 3 * 3 + j / 3][d] = false;
            }
            false
        }
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut seen_row = vec![vec![false; 10]; 10];
        let mut seen_column = vec![vec![false; 10]; 10];
        let mut seen_grid = vec![vec![false; 10]; 10];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let d = board[i][j] as usize - b'0' as usize;
                    seen_row[i][d] = true;
                    seen_column[j][d] = true;
                    seen_grid[i / 3 * 3 + j / 3][d] = true;
                }
            }
        }

        Self::dfs(0, 0, board, &mut seen_row, &mut seen_column, &mut seen_grid);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );

        let mut board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        Solution::solve_sudoku(&mut board);
    }
}
