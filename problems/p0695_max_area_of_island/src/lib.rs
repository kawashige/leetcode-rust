pub struct Solution {}

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let mut count = 0;
                    grid[i][j] = 0;
                    let mut stack = vec![(i as i32, j as i32)];
                    while let Some((r, c)) = stack.pop() {
                        count += 1;
                        for (m_r, m_c) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                            let (a_r, a_c) = (r + m_r, c + m_c);
                            if a_r < 0
                                || a_c < 0
                                || grid.len() - 1 < a_r as usize
                                || grid[0].len() - 1 < a_c as usize
                                || grid[a_r as usize][a_c as usize] == 0
                            {
                                continue;
                            }
                            stack.push((a_r, a_c));
                            grid[a_r as usize][a_c as usize] = 0;
                        }
                    }
                    result = std::cmp::max(result, count);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0695() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );

        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
            0
        );

        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1]
            ]),
            4
        );
    }
}
