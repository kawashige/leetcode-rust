pub struct Solution {}

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    let mut count = 0;
                    for d in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                        let mut p = (i as i32 + d.0, j as i32 + d.1);
                        while 0 <= p.0
                            && p.0 < 8
                            && 0 <= p.1
                            && p.1 < 8
                            && board[p.0 as usize][p.1 as usize] != 'B'
                        {
                            if board[p.0 as usize][p.1 as usize] == 'p' {
                                count += 1;
                                break;
                            }
                            p = (p.0 + d.0, p.1 + d.1);
                        }
                    }
                    return count;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0999() {
        assert_eq!(
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
        );
        assert_eq!(
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            0
        );
        assert_eq!(
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
        );
    }
}
