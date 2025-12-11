pub struct Solution {}

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut x_min = vec![std::i32::MAX; n as usize + 1];
        let mut x_max = vec![std::i32::MIN; n as usize + 1];
        let mut y_min = vec![std::i32::MAX; n as usize + 1];
        let mut y_max = vec![std::i32::MIN; n as usize + 1];

        for b in &buildings {
            x_min[b[1] as usize] = x_min[b[1] as usize].min(b[0]);
            x_max[b[1] as usize] = x_max[b[1] as usize].max(b[0]);
            y_min[b[0] as usize] = y_min[b[0] as usize].min(b[1]);
            y_max[b[0] as usize] = y_max[b[0] as usize].max(b[1]);
        }

        buildings
            .into_iter()
            .filter(|b| {
                x_min[b[1] as usize] != std::i32::MAX
                    && x_min[b[1] as usize] < b[0]
                    && x_max[b[1] as usize] != std::i32::MIN
                    && b[0] < x_max[b[1] as usize]
                    && y_min[b[0] as usize] != std::i32::MAX
                    && y_min[b[0] as usize] < b[1]
                    && y_max[b[0] as usize] != std::i32::MIN
                    && b[1] < y_max[b[0] as usize]
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3531() {
        assert_eq!(
            Solution::count_covered_buildings(
                3,
                vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]]
            ),
            1
        );
        assert_eq!(
            Solution::count_covered_buildings(
                3,
                vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]
            ),
            0
        );
        assert_eq!(
            Solution::count_covered_buildings(
                5,
                vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]]
            ),
            1
        );
    }
}
