use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut seen = vec![vec![false; is_water[0].len()]; is_water.len()];

        for i in 0..is_water.len() {
            for j in 0..is_water[0].len() {
                if is_water[i][j] == 1 {
                    seen[i][j] = true;
                    for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                        let (x, y) = (i as i32 + dx, j as i32 + dy);
                        if x < 0
                            || y < 0
                            || is_water.len() as i32 == x
                            || is_water[0].len() as i32 == y
                        {
                            continue;
                        }
                        queue.push_back(((x as usize, y as usize), 1));
                    }
                }
            }
        }

        let mut matrix = vec![vec![0; is_water[0].len()]; is_water.len()];

        while let Some(((i, j), h)) = queue.pop_front() {
            if seen[i][j] {
                continue;
            }
            seen[i][j] = true;
            matrix[i][j] = h;

            for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (x, y) = (i as i32 + dx, j as i32 + dy);
                if x < 0
                    || y < 0
                    || is_water.len() as i32 == x
                    || is_water[0].len() as i32 == y
                    || seen[x as usize][y as usize]
                {
                    continue;
                }
                queue.push_back(((x as usize, y as usize), h + 1));
            }
        }

        matrix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1765() {
        assert_eq!(
            Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]),
            vec![vec![1, 0], vec![2, 1]]
        );
        assert_eq!(
            Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]),
            vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
        );
    }
}
