pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn search(board: &Vec<Vec<char>>, chars: Vec<char>, routes: Vec<(usize, usize)>) -> bool {
            if chars.len() == 0 {
                return true;
            }
            let last = routes.last().unwrap();
            let mut points = Vec::new();
            // Up
            if 0 < last.1
                && board[last.0][last.1 - 1] == chars[0]
                && !routes.contains(&(last.0, last.1 - 1))
            {
                points.push((last.0, last.1 - 1));
            }
            // down
            if last.0 < board.len() - 1
                && board[last.0 + 1][last.1] == chars[0]
                && !routes.contains(&(last.0 + 1, last.1))
            {
                points.push((last.0 + 1, last.1));
            }
            // right
            if last.1 < board[0].len() - 1
                && board[last.0][last.1 + 1] == chars[0]
                && !routes.contains(&(last.0, last.1 + 1))
            {
                points.push((last.0, last.1 + 1));
            }
            // left
            if 0 < last.0
                && board[last.0 - 1][last.1] == chars[0]
                && !routes.contains(&(last.0 - 1, last.1))
            {
                points.push((last.0 - 1, last.1));
            }

            for p in points.iter() {
                let mut new_routes = routes.clone();
                new_routes.push((p.0, p.1));
                if search(board, chars[1..].to_vec(), new_routes) {
                    return true;
                }
            }
            false
        }
        let chars = word.chars().collect::<Vec<char>>();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == chars[0] {
                    if search(&board, chars[1..].to_vec(), vec![(i, j)]) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0079() {
        let input = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        assert!(Solution::exist(input.clone(), "ABCCED".to_string()));
        assert!(Solution::exist(input.clone(), "SEE".to_string()));
        assert!(!Solution::exist(input.clone(), "ABCD".to_string()));
        assert!(!Solution::exist(input, "CEC".to_string()));
    }
}
