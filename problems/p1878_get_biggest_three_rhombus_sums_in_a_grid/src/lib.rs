use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut set = HashSet::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                set.insert(grid[i][j]);

                for k in 1.. {
                    if grid.len() - 1 < i + k * 2 || j < k || grid[0].len() - 1 < j + k {
                        break;
                    }

                    let mut sum = 0;
                    let (mut r, mut c) = (i, j);

                    for (dr, dc) in [(1, -1), (1, 1), (-1, 1), (-1, -1)].iter() {
                        for _ in 1..=k {
                            r = (r as i32 + dr) as usize;
                            c = (c as i32 + dc) as usize;
                            sum += grid[r][c];
                        }
                    }

                    set.insert(sum);
                }
            }
        }

        let mut sums = set.into_iter().collect::<Vec<_>>();
        sums.sort_unstable_by(|a, b| b.cmp(&a));
        sums[..sums.len().min(3)].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1878() {
        assert_eq!(
            Solution::get_biggest_three(vec![
                vec![3, 4, 5, 1, 3],
                vec![3, 3, 4, 2, 3],
                vec![20, 30, 200, 40, 10],
                vec![1, 5, 5, 4, 1],
                vec![4, 3, 2, 2, 5],
            ]),
            vec![228, 216, 211]
        );
        assert_eq!(
            Solution::get_biggest_three(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![20, 9, 8]
        );
        assert_eq!(Solution::get_biggest_three(vec![vec![7, 7, 7]]), vec![7]);
    }
}
