pub struct Solution {}

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut sub_island_count = 0;
        let mut seen = vec![vec![false; grid2[0].len()]; grid2.len()];
        for i in 0..grid2.len() {
            for j in 0..grid2[0].len() {
                if seen[i][j] || grid2[i][j] == 0 {
                    continue;
                }
                let mut stack = vec![(i, j)];
                let mut is_subisland = true;
                while let Some((k, l)) = stack.pop() {
                    if seen[k][l] {
                        continue;
                    }
                    if grid1[k][l] == 0 {
                        is_subisland = false;
                    }
                    seen[k][l] = true;
                    for (dr, dc) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                        let (r, c) = (k as i32 + dr, l as i32 + dc);
                        if r < 0
                            || c < 0
                            || grid2.len() as i32 <= r
                            || grid2[0].len() as i32 <= c
                            || grid2[r as usize][c as usize] == 0
                            || seen[r as usize][c as usize]
                        {
                            continue;
                        }
                        stack.push((r as usize, c as usize));
                    }
                }

                if is_subisland {
                    sub_island_count += 1;
                }
            }
        }

        sub_island_count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1905() {
        assert_eq!(
            Solution::count_sub_islands(
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 1, 1]
                ],
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 0, 1, 1, 1],
                    vec![0, 1, 0, 0, 0],
                    vec![1, 0, 1, 1, 0],
                    vec![0, 1, 0, 1, 0]
                ]
            ),
            3
        );
        assert_eq!(
            Solution::count_sub_islands(
                vec![
                    vec![1, 0, 1, 0, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1],
                    vec![1, 0, 1, 0, 1]
                ],
                vec![
                    vec![0, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1],
                    vec![0, 1, 0, 1, 0],
                    vec![0, 1, 0, 1, 0],
                    vec![1, 0, 0, 0, 1]
                ]
            ),
            2
        );
    }
}
