pub struct Solution {}

impl Solution {
    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() {
            // left to right
            let mut index = 0;
            for j in 0..board[0].len() {
                if board[i][j] == '#' {
                    index = 0;
                    continue;
                }
                if index < word.len()
                    && (board[i][j] == ' ' || board[i][j] as u8 == word.as_bytes()[index])
                {
                    index += 1;
                    if index == word.len() && (j == board[0].len() - 1 || board[i][j + 1] == '#') {
                        return true;
                    }
                } else {
                    index = word.len();
                }
            }

            // right to left
            let mut index = 0;
            for j in (0..board[0].len()).rev() {
                if board[i][j] == '#' {
                    index = 0;
                    continue;
                }
                if index < word.len()
                    && (board[i][j] == ' ' || board[i][j] as u8 == word.as_bytes()[index])
                {
                    index += 1;
                    if index == word.len() && (j == 0 || board[i][j - 1] == '#') {
                        return true;
                    }
                } else {
                    index = word.len();
                }
            }
        }
        for j in 0..board[0].len() {
            // top to bottom
            let mut index = 0;
            for i in 0..board.len() {
                if board[i][j] == '#' {
                    index = 0;
                    continue;
                }
                if index < word.len()
                    && (board[i][j] == ' ' || board[i][j] as u8 == word.as_bytes()[index])
                {
                    index += 1;
                    if index == word.len() && (i == board.len() - 1 || board[i + 1][j] == '#') {
                        return true;
                    }
                } else {
                    index = word.len();
                }
            }

            // bottom to top
            let mut index = 0;
            for i in (0..board.len()).rev() {
                if board[i][j] == '#' {
                    index = 0;
                    continue;
                }
                if index < word.len()
                    && (board[i][j] == ' ' || board[i][j] as u8 == word.as_bytes()[index])
                {
                    index += 1;
                    if index == word.len() && (i == 0 || board[i - 1][j] == '#') {
                        return true;
                    }
                } else {
                    index = word.len();
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2018() {
        assert!(Solution::place_word_in_crossword(
            vec![
                vec!['#', ' ', '#'],
                vec!['#', ' ', '#'],
                vec!['#', ' ', 'c']
            ],
            "ca".to_string()
        ));
        assert!(Solution::place_word_in_crossword(
            vec![
                vec!['#', ' ', '#'],
                vec![' ', ' ', '#'],
                vec!['#', 'c', ' ']
            ],
            "abc".to_string()
        ));
        assert!(!Solution::place_word_in_crossword(
            vec![
                vec![' ', '#', 'a'],
                vec![' ', '#', 'c'],
                vec![' ', '#', 'a']
            ],
            "ac".to_string()
        ));
        assert!(Solution::place_word_in_crossword(
            vec![
                vec!['#', ' ', '#'],
                vec![' ', ' ', '#'],
                vec!['#', ' ', 'c']
            ],
            "ca".to_string()
        ));
    }
}
