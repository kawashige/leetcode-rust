pub struct Solution {}

impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let check_move = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];
        check_move.iter().any(|(dr, dc)| {
            let (mut r, mut c) = (r_move + dr, c_move + dc);
            let mut count = 0;
            while 0 <= r
                && 0 <= c
                && r < board.len() as i32
                && c < board[0].len() as i32
                && board[r as usize][c as usize] != '.'
            {
                if board[r as usize][c as usize] == color {
                    return 0 < count;
                }
                count += 1;
                r += dr;
                c += dc;
            }
            false
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1958() {
        assert!(Solution::check_move(
            vec![
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.']
            ],
            4,
            3,
            'B'
        ));
        assert!(!Solution::check_move(
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'B', '.', '.', 'W', '.', '.', '.'],
                vec!['.', '.', 'W', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', 'B', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', 'B', 'W', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', 'W', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', 'B']
            ],
            4,
            4,
            'W'
        ));
    }
}
