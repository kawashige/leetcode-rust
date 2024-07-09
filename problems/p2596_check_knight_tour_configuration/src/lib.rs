pub struct Solution {}

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid[0][0] != 0 {
            return false;
        }
        let mut moves = Vec::with_capacity(grid.len() * grid[0].len());
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                moves.push((grid[i][j], (i as i32, j as i32)));
            }
        }
        moves.sort_unstable();

        for i in 1..moves.len() {
            let diff = (
                (moves[i].1 .0 - moves[i - 1].1 .0).abs(),
                (moves[i].1 .1 - moves[i - 1].1 .1).abs(),
            );
            if !(diff == (1, 2) || diff == (2, 1)) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2596() {
        assert!(!Solution::check_valid_grid(vec![
            vec![24, 11, 22, 17, 4],
            vec![21, 16, 5, 12, 9],
            vec![6, 23, 10, 3, 18],
            vec![15, 20, 1, 8, 13],
            vec![0, 7, 14, 19, 2]
        ]));
        assert!(Solution::check_valid_grid(vec![
            vec![0, 11, 16, 5, 20],
            vec![17, 4, 19, 10, 15],
            vec![12, 1, 8, 21, 6],
            vec![3, 18, 23, 14, 9],
            vec![24, 13, 2, 7, 22]
        ]));
        assert!(!Solution::check_valid_grid(vec![
            vec![0, 3, 6],
            vec![5, 8, 1],
            vec![2, 7, 4]
        ]));
    }
}
