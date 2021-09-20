pub struct Solution {}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];

        for (i, m) in moves.iter().enumerate() {
            board[m[0] as usize][m[1] as usize] = if i % 2 == 0 { 'A' } else { 'B' };
        }

        if (0..3).any(|i| (1..3).all(|j| board[i][0] == board[i][j]))
            || (0..3).any(|i| (1..3).all(|j| board[0][i] == board[j][i]))
            || (board[0][0] == board[1][1] && board[0][0] == board[2][2])
            || (board[0][2] == board[1][1] && board[0][2] == board[2][0])
        {
            if moves.len() % 2 == 1 {
                "A"
            } else {
                "B"
            }
        } else if moves.len() == 9 {
            "Draw"
        } else {
            "Pending"
        }
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ]),
            "A".to_string()
        );
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0]
            ]),
            "B".to_string()
        );
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ]),
            "Draw".to_string()
        );
        assert_eq!(
            Solution::tictactoe(vec![vec![0, 0], vec![1, 1]]),
            "Pending".to_string()
        );
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 2],
                vec![1, 0],
                vec![2, 0],
                vec![0, 1],
                vec![1, 2],
                vec![1, 1],
                vec![0, 2]
            ]),
            "B".to_string()
        );
    }
}
