use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_moves(matrix: Vec<String>) -> i32 {
        let matrix = matrix
            .into_iter()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut letters = vec![vec![]; 26];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j].is_ascii_alphabetic() {
                    letters[(matrix[i][j] as u8 - b'A') as usize].push((i, j));
                }
            }
        }

        let mut seen = vec![vec![false; matrix[0].len()]; matrix.len()];
        let mut queue = VecDeque::new();
        queue.push_back((0, (0, 0)));

        while let Some((c, (i, j))) = queue.pop_front() {
            for (x, y) in if matrix[i][j].is_ascii_alphabetic() {
                letters[(matrix[i][j] as u8 - b'A') as usize].clone()
            } else {
                vec![(i, j)]
            } {
                if (x, y) == (matrix.len() - 1, matrix[0].len() - 1) {
                    return c;
                }
                if seen[x][y] {
                    continue;
                }
                seen[x][y] = true;

                for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                    let (new_i, new_j) = (x as i32 + di, y as i32 + dj);
                    if !(0..matrix.len() as i32).contains(&new_i)
                        || !(0..matrix[0].len() as i32).contains(&new_j)
                        || matrix[new_i as usize][new_j as usize] == '#'
                    {
                        continue;
                    }
                    queue.push_back((c + 1, (new_i as usize, new_j as usize)));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3552() {
        assert_eq!(
            Solution::min_moves(vec![
                "A..".to_string(),
                ".A.".to_string(),
                "...".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::min_moves(vec![
                ".#...".to_string(),
                ".#.#.".to_string(),
                ".#.#.".to_string(),
                "...#.".to_string()
            ]),
            13
        );
    }
}
