pub struct Solution {}

impl Solution {
    pub fn swap(s: i32, board: &mut Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..board[0].len() {
            let target = if i % 2 == 0 { s } else { (s + 1) % 2 };
            if board[0][i] != target {
                if let Some(j) = ((i + 1)..board[0].len())
                    .step_by(2)
                    .find(|j| board[0][*j] == target)
                {
                    for k in 0..board.len() {
                        board[k].swap(i, j);
                    }
                    count += 1;
                } else {
                    return -1;
                }
            }
        }

        if (1..board.len()).all(|i| (1..board[0].len()).all(|j| board[i][j] != board[i][j - 1])) {
            let zero = Self::swap_row(0, &mut board.clone());
            let one = Self::swap_row(1, &mut board.clone());
            if zero == -1 && one == -1 {
                -1
            } else if zero == -1 {
                one + count
            } else if one == -1 {
                zero + count
            } else {
                one.min(zero) + count
            }
        } else {
            -1
        }
    }

    pub fn swap_row(s: i32, board: &mut Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..board.len() {
            let target = if i % 2 == 0 { s } else { (s + 1) % 2 };
            if board[i][0] != target {
                if let Some(j) = ((i + 1)..board.len())
                    .step_by(2)
                    .find(|j| board[*j][0] == target)
                {
                    board[i][0] = target;
                    board[j][0] = (target + 1) % 2;
                    count += 1;
                } else {
                    return -1;
                }
            }
        }
        count
    }

    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let zero = Self::swap(0, &mut board.clone());
        let one = Self::swap(1, &mut board.clone());
        if zero == -1 && one == -1 {
            -1
        } else if zero == -1 {
            one
        } else if one == -1 {
            zero
        } else {
            one.min(zero)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day26() {
        assert_eq!(
            Solution::moves_to_chessboard(vec![
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![1, 0, 0, 1],
                vec![1, 0, 0, 1]
            ]),
            2
        );
        assert_eq!(
            Solution::moves_to_chessboard(vec![vec![0, 1], vec![1, 0]]),
            0
        );
        assert_eq!(
            Solution::moves_to_chessboard(vec![vec![1, 0], vec![1, 0]]),
            -1
        );
    }
}
