pub struct Solution {}

impl Solution {
    pub fn calc_area(
        left_top: (usize, usize),
        right_bottom: (usize, usize),
        grid: &Vec<Vec<i32>>,
    ) -> i32 {
        let mut r = [std::usize::MAX, std::usize::MIN];
        let mut c = [std::usize::MAX, std::usize::MIN];
        for i in left_top.0..=right_bottom.0 {
            for j in left_top.1..=right_bottom.1 {
                if grid[i][j] == 1 {
                    r[0] = r[0].min(i);
                    r[1] = r[1].max(i);
                    c[0] = c[0].min(j);
                    c[1] = c[1].max(j);
                }
            }
        }
        if r[0] == std::usize::MAX {
            return 1;
        }
        ((r[1] - r[0] + 1) * (c[1] - c[0] + 1)) as i32
    }

    pub fn calc_split_area(
        left_top: (usize, usize),
        right_bottom: (usize, usize),
        grid: &Vec<Vec<i32>>,
    ) -> i32 {
        let mut result = std::i32::MAX;
        // horizontal
        for i in left_top.0..right_bottom.0 {
            result = result.min(
                Self::calc_area(left_top, (i, right_bottom.1), &grid)
                    + Self::calc_area((i + 1, left_top.1), right_bottom, &grid),
            );
        }

        // vertilal
        for j in left_top.1..right_bottom.1 {
            result = result.min(
                Self::calc_area(left_top, (right_bottom.0, j), &grid)
                    + Self::calc_area((left_top.0, j + 1), right_bottom, &grid),
            );
        }

        result
    }

    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = std::i32::MAX;

        // horizontal
        for i in 0..grid.len() - 1 {
            let rec1 = ((0, 0), (i, grid[0].len() - 1));
            let rec2 = ((i + 1, 0), (grid.len() - 1, grid[0].len() - 1));
            result = result.min(
                Self::calc_area(rec1.0, rec1.1, &grid)
                    + Self::calc_split_area(rec2.0, rec2.1, &grid),
            );
            result = result.min(
                Self::calc_area(rec2.0, rec2.1, &grid)
                    + Self::calc_split_area(rec1.0, rec1.1, &grid),
            );
        }

        // vertical
        for j in 0..grid[0].len() - 1 {
            let rec1 = ((0, 0), (grid.len() - 1, j));
            let rec2 = ((0, j + 1), (grid.len() - 1, grid[0].len() - 1));
            result = result.min(
                Self::calc_area(rec1.0, rec1.1, &grid)
                    + Self::calc_split_area(rec2.0, rec2.1, &grid),
            );
            result = result.min(
                Self::calc_area(rec2.0, rec2.1, &grid)
                    + Self::calc_split_area(rec1.0, rec1.1, &grid),
            );
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3197() {
        assert_eq!(
            Solution::minimum_sum(vec![
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![1, 1, 0]
            ]),
            3
        );
        assert_eq!(Solution::minimum_sum(vec![vec![0, 1], vec![1, 1]]), 3);
        assert_eq!(Solution::minimum_sum(vec![vec![1, 0, 1], vec![1, 1, 1]]), 5);
        assert_eq!(
            Solution::minimum_sum(vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]]),
            5
        );
    }
}
