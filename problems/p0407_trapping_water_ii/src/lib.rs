pub struct Solution {}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut height = vec![vec![std::i32::MAX; height_map[0].len()]; height_map.len()];
        let mut stack = Vec::new();
        for j in 0..height_map[0].len() {
            stack.push(((0, j), height_map[0][j]));
            stack.push((
                (height_map.len() - 1, j),
                height_map[height_map.len() - 1][j],
            ));
        }
        for i in 0..height_map.len() {
            stack.push(((i, 0), height_map[i][0]));
            stack.push((
                (i, height_map[0].len() - 1),
                height_map[i][height_map[0].len() - 1],
            ))
        }

        while let Some(((i, j), h)) = stack.pop() {
            if height[i][j] <= h {
                continue;
            }
            height[i][j] = h.max(height_map[i][j]);
            for (di, dj) in [(-1, 0), (0, 1), (0, -1), (1, 0)].iter() {
                let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
                if !(0..height_map.len() as i32).contains(&new_i)
                    || !(0..height_map[0].len() as i32).contains(&new_j)
                {
                    continue;
                }
                stack.push(((new_i as usize, new_j as usize), height[i][j]));
            }
        }
        println!("{:?}", height);

        let mut result = 0;
        for i in 1..height_map.len() - 1 {
            for j in 1..height_map[0].len() - 1 {
                result += (height[i][j] - height_map[i][j]).max(0);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0407() {
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![9, 9, 9, 9, 9],
                vec![9, 2, 1, 2, 9],
                vec![9, 2, 8, 2, 9],
                vec![9, 2, 3, 2, 9],
                vec![9, 9, 9, 9, 9]
            ]),
            57
        );
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![12, 13, 1, 12],
                vec![13, 4, 13, 12],
                vec![13, 8, 10, 12],
                vec![12, 13, 12, 12],
                vec![13, 13, 13, 13]
            ]),
            14
        );
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1]
            ]),
            4
        );
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![3, 3, 3, 3, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 2, 1, 2, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 3, 3, 3, 3]
            ]),
            10
        );
    }
}
