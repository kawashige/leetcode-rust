use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let classroom = classroom
            .into_iter()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (mut sx, mut sy) = (0, 0);
        let mut litters = Vec::new();

        for i in 0..classroom.len() {
            for j in 0..classroom[0].len() {
                if classroom[i][j] == 'S' {
                    (sx, sy) = (i, j);
                } else if classroom[i][j] == 'L' {
                    litters.push((i, j));
                }
            }
        }
        let litter_count = litters.len() as u32;

        let mut best_energy =
            vec![vec![vec![-1; 2_usize.pow(litter_count)]; classroom[0].len()]; classroom.len()];
        let mut queue = VecDeque::new();
        queue.push_back((sx, sy, 0, energy, 0));
        let goal = 2_usize.pow(litter_count) - 1;

        while let Some((x, y, mask, e, steps)) = queue.pop_front() {
            if mask == goal {
                return steps;
            }
            if e <= best_energy[x][y][mask] {
                continue;
            }
            best_energy[x][y][mask] = e;
            let new_e = if classroom[x][y] == 'R' { energy } else { e };
            if new_e == 0 {
                continue;
            }

            for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
                if !(0..classroom.len() as i32).contains(&new_x)
                    || !(0..classroom[0].len() as i32).contains(&new_y)
                    || classroom[new_x as usize][new_y as usize] == 'X'
                {
                    continue;
                }
                queue.push_back((
                    new_x as usize,
                    new_y as usize,
                    if classroom[new_x as usize][new_y as usize] == 'L' {
                        mask | 1
                            << (0..litters.len())
                                .find(|i| litters[*i] == (new_x as usize, new_y as usize))
                                .unwrap()
                    } else {
                        mask
                    },
                    new_e - 1,
                    steps + 1,
                ));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3568() {
        assert_eq!(
            Solution::min_moves(vec!["S.".to_string(), "XL".to_string()], 2),
            2
        );
        assert_eq!(
            Solution::min_moves(vec!["LS".to_string(), "RL".to_string()], 4),
            3
        );
        assert_eq!(
            Solution::min_moves(vec!["L.S".to_string(), "RXL".to_string()], 3),
            -1
        );
    }
}
