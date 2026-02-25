pub struct Solution {}

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut result = vec![vec![0; grid[0].len() + 1 - k]; grid.len() + 1 - k];

        for i in 0..grid.len() + 1 - k {
            for j in 0..grid[0].len() + 1 - k {
                let mut values = Vec::new();
                for x in i..i + k {
                    for y in j..j + k {
                        values.push(grid[x][y]);
                    }
                }
                values.sort_unstable();
                if values.len() == 1 {
                    result[i][j] = 0;
                } else {
                    result[i][j] = (1..values.len())
                        .map(|l| (values[l - 1] - values[l]).abs())
                        .filter(|d| &0 < d)
                        .min()
                        .unwrap_or(0);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3566() {
        assert_eq!(
            Solution::min_abs_diff(vec![vec![1, 8], vec![3, -2]], 2),
            vec![vec![2]]
        );
        assert_eq!(
            Solution::min_abs_diff(vec![vec![3, -1]], 1),
            vec![vec![0, 0]]
        );
        assert_eq!(
            Solution::min_abs_diff(vec![vec![1, -2, 3], vec![2, 3, 5]], 2),
            vec![vec![1, 2]]
        );
    }
}
