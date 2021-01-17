pub struct Solution {}

impl Solution {
    pub fn adjacent_positions(i: usize, j: usize, r: usize, c: usize) -> Vec<Vec<usize>> {
        vec![
            (-1_i32, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(x, y)| (x + i as i32, y + j as i32))
        .filter(|(x, y)| &0 <= x && x < &(r as i32) && &0 <= y && y < &(c as i32))
        .map(|(x, y)| vec![x as usize, y as usize])
        .collect::<Vec<Vec<usize>>>()
    }

    pub fn recurse(board: &mut Vec<Vec<char>>, click: Vec<usize>) {
        if board[click[0]][click[1]] == 'M' {
            board[click[0]][click[1]] = 'X';
        } else {
            let adjacents =
                Self::adjacent_positions(click[0], click[1], board.len(), board[0].len());
            let mine_count = adjacents
                .iter()
                .filter(|v| board[v[0]][v[1]] == 'M')
                .count() as u8;
            if mine_count == 0 {
                board[click[0]][click[1]] = 'B';
                for v in adjacents {
                    if board[v[0]][v[1]] == 'E' {
                        Self::recurse(board, v);
                    }
                }
            } else {
                board[click[0]][click[1]] = (mine_count + b'0') as char;
            }
        }
    }

    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        Self::recurse(&mut board, click.iter().map(|c| *c as usize).collect());
        board
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_529() {
        assert_eq!(
            vec![
                vec!['B', '1', 'E', '1', 'B'],
                vec!['B', '1', 'M', '1', 'B'],
                vec!['B', '1', '1', '1', 'B'],
                vec!['B', 'B', 'B', 'B', 'B']
            ],
            Solution::update_board(
                vec![
                    vec!['E', 'E', 'E', 'E', 'E'],
                    vec!['E', 'E', 'M', 'E', 'E'],
                    vec!['E', 'E', 'E', 'E', 'E'],
                    vec!['E', 'E', 'E', 'E', 'E']
                ],
                vec![3, 0]
            )
        )
    }
}
