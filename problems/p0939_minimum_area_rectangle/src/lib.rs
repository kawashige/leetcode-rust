use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let set = points.iter().fold(HashSet::new(), |mut set, p| {
            set.insert((p[0], p[1]));
            set
        });

        let mut min = std::i32::MAX;

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                if points[i][0] == points[j][0] || points[i][1] == points[j][1] {
                    continue;
                }

                if set.contains(&(points[i][0], points[j][1]))
                    && set.contains(&(points[j][0], points[i][1]))
                {
                    min = min.min(
                        (points[i][0] - points[j][0]).abs() * (points[i][1] - points[j][1]).abs(),
                    );
                }
            }
        }

        if min == std::i32::MAX {
            0
        } else {
            min
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0939() {
        assert_eq!(
            Solution::min_area_rect(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![2, 2]
            ]),
            4
        );
        assert_eq!(
            Solution::min_area_rect(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![4, 1],
                vec![4, 3]
            ]),
            2
        );
    }
}
