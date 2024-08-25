pub struct Solution {}

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut move_ok = vec![true; grid.len()];
        for j in 1..grid[0].len() {
            let mut new_move_ok = vec![false; grid.len()];
            let mut found = false;
            for i in 0..grid.len() {
                for di in [-1, 0, 1].iter() {
                    let prev_i = i as i32 + di;
                    if !(0..grid.len() as i32).contains(&prev_i)
                        || !move_ok[prev_i as usize]
                        || grid[i][j] <= grid[prev_i as usize][j - 1]
                    {
                        continue;
                    }
                    new_move_ok[i] = true;
                    found = true;
                }
            }
            if !found {
                return j as i32 - 1;
            }
            move_ok = new_move_ok;
        }
        grid[0].len() as i32 - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2684() {
        assert_eq!(
            Solution::max_moves(vec![
                vec![2, 4, 3, 5],
                vec![5, 4, 9, 3],
                vec![3, 4, 2, 11],
                vec![10, 9, 13, 15]
            ]),
            3
        );
        assert_eq!(
            Solution::max_moves(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]]),
            0
        );
    }
}
