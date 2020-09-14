pub struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        use std::collections::HashSet;
        let mut border = HashSet::new();
        let mut not_border = HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'O' {
                    if i == 0 || i == board.len() - 1 || j == 0 || j == board[0].len() - 1 {
                        border.insert((i, j));
                    } else {
                        not_border.insert((i, j));
                    }
                }
            }
        }
        let moves: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        while border.len() > 0 {
            let mut new_border = HashSet::new();
            for b in border {
                for m in moves
                    .iter()
                    .map(|m| (b.0 as i32 + m.0, b.1 as i32 + m.1))
                    .filter(|(x, y)| {
                        0 < *x
                            && *x < board.len() as i32 - 1
                            && 0 < *y
                            && *y < board[0].len() as i32 - 1
                    })
                {
                    match not_border.take(&(m.0 as usize, m.1 as usize)) {
                        Some(n) => {
                            new_border.insert(n);
                        }
                        None => {}
                    }
                }
            }
            border = new_border;
        }

        for (i, j) in not_border {
            board[i][j] = 'X'
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0130() {
        let mut input = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        Solution::solve(&mut input);

        assert_eq!(
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
            input
        );

        let mut input2 = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
        ];

        Solution::solve(&mut input2);

        assert_eq!(
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'O', 'X'],
            ],
            input2
        );

        let mut input3 = vec![] as Vec<Vec<char>>;
        Solution::solve(&mut input3);
        assert_eq!(vec![] as Vec<Vec<char>>, input3);

        let mut input4 = vec![
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
            vec!['O', 'O', 'O'],
        ];

        Solution::solve(&mut input4);

        assert_eq!(
            vec![
                vec!['O', 'O', 'O'],
                vec!['O', 'O', 'O'],
                vec!['O', 'O', 'O'],
            ],
            input4
        );

        let mut input5 = vec![
            vec!['X', 'O', 'X', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'X'],
        ];

        Solution::solve(&mut input5);

        assert_eq!(
            vec![
                vec!['X', 'O', 'X', 'O', 'X', 'O'],
                vec!['O', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'O'],
                vec!['O', 'X', 'O', 'X', 'O', 'X']
            ],
            input5
        );
    }
}
