pub struct Solution {}

impl Solution {
    pub fn minimum_or(grid: Vec<Vec<i32>>) -> i32 {
        let mut excluded = vec![vec![false; grid[0].len()]; grid.len()];
        let mut result = 0;

        for b in (0..32).rev() {
            if (0..grid.len()).all(|r| {
                grid[r]
                    .iter()
                    .enumerate()
                    .any(|(c, v)| !excluded[r][c] && v & 1 << b == 0)
            }) {
                for r in 0..grid.len() {
                    for c in 0..grid[0].len() {
                        if grid[r][c] & 1 << b != 0 {
                            excluded[r][c] = true;
                        }
                    }
                }
            } else {
                result |= 1 << b;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3858() {
        assert_eq!(Solution::minimum_or(vec![vec![1, 5], vec![2, 4]]), 3);
        assert_eq!(Solution::minimum_or(vec![vec![3, 5], vec![6, 4]]), 5);
        assert_eq!(Solution::minimum_or(vec![vec![7, 9, 8]]), 7);
    }
}

fn main() {}
