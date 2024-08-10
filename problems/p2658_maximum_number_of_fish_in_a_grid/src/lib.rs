pub struct Solution {}

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        let mut max_count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 || seen[i][j] {
                    continue;
                }
                let mut stack = vec![(i, j)];
                let mut count = 0;
                while let Some((i, j)) = stack.pop() {
                    if seen[i][j] {
                        continue;
                    }
                    count += grid[i][j];
                    seen[i][j] = true;
                    for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                        let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                        if !(0..grid.len() as i32).contains(&new_i)
                            || !(0..grid[0].len() as i32).contains(&new_j)
                            || grid[new_i as usize][new_j as usize] == 0
                            || seen[new_i as usize][new_j as usize]
                        {
                            continue;
                        }
                        stack.push((new_i as usize, new_j as usize));
                    }
                }
                max_count = max_count.max(count)
            }
        }

        max_count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2658() {
        assert_eq!(
            Solution::find_max_fish(vec![
                vec![0, 2, 1, 0],
                vec![4, 0, 0, 3],
                vec![1, 0, 0, 4],
                vec![0, 3, 2, 0]
            ]),
            7
        );
        assert_eq!(
            Solution::find_max_fish(vec![
                vec![1, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1]
            ]),
            1
        );
    }
}
