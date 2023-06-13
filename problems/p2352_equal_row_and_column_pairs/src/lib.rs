pub struct Solution {}

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                let mut is_equal = true;
                for k in 0..grid.len() {
                    if grid[i][k] != grid[k][j] {
                        is_equal = false;
                        break;
                    }
                }
                if is_equal {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2352() {
        assert_eq!(
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
        assert_eq!(
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        );
    }
}
