pub struct Solution {}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let moves = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let live_neighbors = moves.iter().fold(0, |count, m| {
                    let neighbor = [i as i32 + m[0], j as i32 + m[1]];
                    if 0 <= neighbor[0]
                        && neighbor[0] < board.len() as i32
                        && 0 <= neighbor[1]
                        && neighbor[1] < board[0].len() as i32
                        && (board[neighbor[0] as usize][neighbor[1] as usize] == 1
                            || board[neighbor[0] as usize][neighbor[1] as usize] == 2)
                    {
                        count + 1
                    } else {
                        count
                    }
                });
                if board[i as usize][j as usize] == 1 && (live_neighbors < 2 || 3 < live_neighbors)
                {
                    board[i as usize][j as usize] = 2;
                } else if board[i as usize][j as usize] == 0 && live_neighbors == 3 {
                    board[i as usize][j as usize] = 3;
                }
            }
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 2 {
                    board[i][j] = 0;
                } else if board[i][j] == 3 {
                    board[i][j] = 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0289() {
        let mut input = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut input);
        let result = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
        assert_eq!(input, result);
    }
}
