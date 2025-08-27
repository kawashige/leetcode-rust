pub struct Solution {}

impl Solution {
    const DIRECTION: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

    pub fn recurse(
        i: i32,
        j: i32,
        d: usize,
        turned: bool,
        grid: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if dp[i as usize][j as usize][d][turned as usize] != std::i32::MIN {
            return dp[i as usize][j as usize][d][turned as usize];
        }
        let mut direction_index = vec![d];
        if !turned {
            direction_index.push((d + 1) % Self::DIRECTION.len());
        }

        let mut max = 0;

        for k in direction_index {
            let (new_i, new_j) = (i + Self::DIRECTION[k].0, j + Self::DIRECTION[k].1);
            if !(0..grid.len() as i32).contains(&new_i)
                || !(0..grid[0].len() as i32).contains(&new_j)
                || grid[new_i as usize][new_j as usize]
                    != if grid[i as usize][j as usize] == 2 {
                        0
                    } else {
                        2
                    }
            {
                continue;
            }
            max = max.max(Self::recurse(
                new_i,
                new_j,
                k,
                if k == d { turned } else { true },
                &grid,
                dp,
            ));
        }

        dp[i as usize][j as usize][d][turned as usize] = max + 1;
        dp[i as usize][j as usize][d][turned as usize]
    }

    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![vec![vec![std::i32::MIN; 2]; 4]; grid[0].len()]; grid.len()];
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    result = result.max(1);
                    for k in 0..Self::DIRECTION.len() {
                        let (new_i, new_j) = (
                            i as i32 + Self::DIRECTION[k].0,
                            j as i32 + Self::DIRECTION[k].1,
                        );
                        if !(0..grid.len() as i32).contains(&new_i)
                            || !(0..grid[0].len() as i32).contains(&new_j)
                            || grid[new_i as usize][new_j as usize] != 2
                        {
                            continue;
                        }
                        result =
                            result.max(1 + Self::recurse(new_i, new_j, k, false, &grid, &mut dp));
                    }
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
    fn test_3459() {
        assert_eq!(Solution::len_of_v_diagonal(vec![vec![1]]), 1);
        assert_eq!(
            Solution::len_of_v_diagonal(vec![
                vec![2, 2, 1, 2, 2],
                vec![2, 0, 2, 2, 0],
                vec![2, 0, 1, 1, 0],
                vec![1, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 2]
            ]),
            5
        );
        assert_eq!(
            Solution::len_of_v_diagonal(vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 0, 2, 2, 0],
                vec![2, 0, 1, 1, 0],
                vec![1, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 2]
            ]),
            4
        );
        assert_eq!(
            Solution::len_of_v_diagonal(vec![
                vec![1, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 0],
                vec![2, 0, 0, 0, 0],
                vec![0, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 0]
            ]),
            5
        );
    }
}
