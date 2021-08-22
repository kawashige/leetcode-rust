use std::collections::{BTreeSet, HashMap};

pub struct Solution {}

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        const M: i64 = 1_000_000_007;

        let mut x = BTreeSet::new();
        let mut y = BTreeSet::new();
        for r in &rectangles {
            x.insert(r[0]);
            x.insert(r[2]);
            y.insert(r[1]);
            y.insert(r[3]);
        }

        let x = x.into_iter().collect::<Vec<_>>();
        let x_map = x
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .collect::<HashMap<_, _>>();
        let y = y.into_iter().collect::<Vec<_>>();
        let y_map = y
            .iter()
            .enumerate()
            .map(|(i, y)| (*y, i))
            .collect::<HashMap<_, _>>();

        let mut area = vec![vec![false; y_map.len()]; x_map.len()];

        for r in rectangles {
            for i in x_map[&r[0]]..x_map[&r[2]] {
                for j in y_map[&r[1]]..y_map[&r[3]] {
                    area[i][j] = true;
                }
            }
        }

        let mut total: i64 = 0;
        for i in 0..area.len() {
            for j in 0..area[0].len() {
                if area[i][j] {
                    total += (x[i + 1] - x[i]) as i64 * (y[j + 1] - y[j]) as i64;
                    total %= M;
                }
            }
        }

        total as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day22() {
        assert_eq!(
            Solution::rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]]),
            6
        );
        assert_eq!(
            Solution::rectangle_area(vec![vec![0, 0, 1000000000, 1000000000]]),
            49
        );
    }
}
