pub struct Solution {}

impl Solution {
    pub fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = [[0; 3]; 2];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let k = if (i < grid.len() / 2 && (i == j || i + j == grid.len() - 1))
                    || (i >= grid.len() / 2 && j == grid.len() / 2)
                {
                    0
                } else {
                    1
                };
                count[k][grid[i][j] as usize] += 1;
            }
        }

        let mut result = std::i32::MAX;
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    continue;
                }
                let operations = (0..3)
                    .map(|k| if i == k { 0 } else { count[0][k] })
                    .sum::<i32>()
                    + (0..3)
                        .map(|k| if j == k { 0 } else { count[1][k] })
                        .sum::<i32>();
                result = result.min(operations);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3071() {
        assert_eq!(
            Solution::minimum_operations_to_write_y(vec![
                vec![1, 2, 2],
                vec![1, 1, 0],
                vec![0, 1, 0]
            ]),
            3
        );
        assert_eq!(
            Solution::minimum_operations_to_write_y(vec![
                vec![0, 1, 0, 1, 0],
                vec![2, 1, 0, 1, 2],
                vec![2, 2, 2, 0, 1],
                vec![2, 2, 2, 2, 2],
                vec![2, 1, 2, 2, 2]
            ]),
            12
        );
    }
}
