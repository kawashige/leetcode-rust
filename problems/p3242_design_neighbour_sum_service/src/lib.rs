struct NeighborSum {
    adjacent_sum_: Vec<i32>,
    diagonal_sum_: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut adjacent_sum_ = vec![0; grid.len() * grid.len()];
        let mut diagonal_sum_ = vec![0; grid.len() * grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                    let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                    if !(0..grid.len() as i32).contains(&new_i)
                        || !(0..grid[0].len() as i32).contains(&new_j)
                    {
                        continue;
                    }
                    adjacent_sum_[grid[i][j] as usize] += grid[new_i as usize][new_j as usize];
                }
                for (di, dj) in [(-1, -1), (-1, 1), (1, -1), (1, 1)].iter() {
                    let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                    if !(0..grid.len() as i32).contains(&new_i)
                        || !(0..grid[0].len() as i32).contains(&new_j)
                    {
                        continue;
                    }
                    diagonal_sum_[grid[i][j] as usize] += grid[new_i as usize][new_j as usize];
                }
            }
        }

        Self {
            adjacent_sum_,
            diagonal_sum_,
        }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.adjacent_sum_[value as usize]
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.diagonal_sum_[value as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3242() {
        let mut obj = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        assert_eq!(obj.adjacent_sum(1), 6);
        assert_eq!(obj.adjacent_sum(4), 16);
        assert_eq!(obj.diagonal_sum(4), 16);
        assert_eq!(obj.diagonal_sum(8), 4);

        let mut obj = NeighborSum::new(vec![
            vec![1, 2, 0, 3],
            vec![4, 7, 15, 6],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 5],
        ]);
        assert_eq!(obj.adjacent_sum(15), 23);
        assert_eq!(obj.diagonal_sum(9), 45);
    }
}
