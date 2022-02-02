pub struct Solution {}

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut board = vec![vec![false; 8]; 8];
        for queen in queens {
            board[queen[0] as usize][queen[1] as usize] = true;
        }

        let mut result = Vec::new();
        for (dx, dy) in [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (1, 1),
            (-1, 1),
            (-1, -1),
            (1, -1),
        ]
        .iter()
        {
            let mut x = king[0] + dx;
            let mut y = king[1] + dy;
            while (0..8).contains(&x) && (0..8).contains(&y) {
                if board[x as usize][y as usize] {
                    result.push(vec![x as i32, y as i32]);
                    break;
                }
                x += dx;
                y += dy;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1222() {
        assert_eq!(
            Solution::queens_attackthe_king(
                vec![
                    vec![0, 1],
                    vec![1, 0],
                    vec![4, 0],
                    vec![0, 4],
                    vec![3, 3],
                    vec![2, 4]
                ],
                vec![0, 0]
            ),
            vec![vec![1, 0], vec![0, 1], vec![3, 3]]
        );
        assert_eq!(
            Solution::queens_attackthe_king(
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 4],
                    vec![3, 5],
                    vec![4, 4],
                    vec![4, 5]
                ],
                vec![3, 3]
            ),
            vec![vec![3, 4], vec![4, 4], vec![2, 2]]
        );
        assert_eq!(
            Solution::queens_attackthe_king(
                vec![
                    vec![5, 6],
                    vec![7, 7],
                    vec![2, 1],
                    vec![0, 7],
                    vec![1, 6],
                    vec![5, 1],
                    vec![3, 7],
                    vec![0, 3],
                    vec![4, 0],
                    vec![1, 2],
                    vec![6, 3],
                    vec![5, 0],
                    vec![0, 4],
                    vec![2, 2],
                    vec![1, 1],
                    vec![6, 4],
                    vec![5, 4],
                    vec![0, 0],
                    vec![2, 6],
                    vec![4, 5],
                    vec![5, 2],
                    vec![1, 4],
                    vec![7, 5],
                    vec![2, 3],
                    vec![0, 5],
                    vec![4, 2],
                    vec![1, 0],
                    vec![2, 7],
                    vec![0, 1],
                    vec![4, 6],
                    vec![6, 1],
                    vec![0, 6],
                    vec![4, 3],
                    vec![1, 7]
                ],
                vec![3, 4]
            ),
            vec![
                vec![1, 4],
                vec![5, 4],
                vec![3, 7],
                vec![4, 5],
                vec![1, 6],
                vec![2, 3],
                vec![4, 3],
            ]
        );
    }
}
