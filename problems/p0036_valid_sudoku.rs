pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashMap;

        let size = board.len();

        for i in 0..size {
            let mut count = HashMap::new();
            for j in 0..size {
                if board[i][j] == '.' {
                    continue;
                }
                let c = count.entry(board[i][j]).or_insert(0);
                if *c > 0 {
                    return false;
                }
                *c += 1;
            }
        }

        for i in 0..size {
            let mut count = HashMap::new();
            for j in 0..size {
                if board[j][i] == '.' {
                    continue;
                }
                let c = count.entry(board[j][i]).or_insert(0);
                if *c > 0 {
                    return false;
                }
                *c += 1;
            }
        }

        let mut i = 0;
        while i < size {
            let mut j = 0;
            while j < size {
                let mut count = HashMap::new();
                for k in i..(i + 3) {
                    for l in j..(j + 3) {
                        if board[k][l] == '.' {
                            continue;
                        }
                        let c = count.entry(board[k][l]).or_insert(0);
                        if *c > 0 {
                            return false;
                        }
                        *c += 1;
                    }
                }
                j += 3;
            }
            i += 3;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0036() {
        let input_1 = vec![
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

        assert!(Solution::is_valid_sudoku(input_1));

        let input_2 = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!Solution::is_valid_sudoku(input_2));

        let input_3 = vec![
            vec!['.', '.', '.', '.', '.', '.', '5', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['9', '3', '.', '.', '2', '.', '4', '.', '.'],
            vec!['.', '.', '7', '.', '.', '.', '3', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '3', '4', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '5', '2', '.', '.'],
        ];

        assert!(!Solution::is_valid_sudoku(input_3));
    }
}
