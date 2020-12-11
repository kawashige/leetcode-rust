pub struct Solution {}

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X'
                    && (i == 0 || board[i - 1][j] != 'X')
                    && (j == 0 || board[i][j - 1] != 'X')
                {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0419() {
        assert_eq!(
            2,
            Solution::count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
            ])
        )
    }
}
